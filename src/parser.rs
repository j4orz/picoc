use crate::{
    lexer::{Token, TT},
    rep::{ctl::{Return, Start}, data::{Add, Div, Int, Mul, Sub}, scope::{Scope, ScopeError}, Instr, TypeKind}
};
use std::{io, rc::Rc};
use thiserror::Error;

// NB1. each function in the parser will parse in two ways
//        a. conditionally (SUM/OR): match tokens(first, rest) first.typ { TT::Foo => {}, TT::Bar => {}, TT::Baz => {} }
//        b. assertively (PROD/AND): require(tokens, TT:Foo), eat(tokens, TT:Bar), eat(tokens, TT:Baz)

// NB2. the parser is composed of pure functions less the start instruction and
//      scope/nv instruction so the token stream (r) is threaded throughout

// NB3. generally speaking there are three variants for intermediate representation
//        a. tree ("AST")
//           -> precedence is represented via tree's hierarchy.
//        b. two-tiered (nested) graph of basic blocks of instructions. ("CFG+BB")
//           -> control flow is represented via graph's edges
//        c. single-tiered (flat) graph of instructions ("SoN")
//           -> control flow AND data flow is represented via graph's edges

//      picotile parses the concrete syntax into (c) because TODO. see optimizer
//      for more details. this means that the total ordering of straightline
//      code (vec<list>) is relaxed to a partial order of a graph

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("scope")]
    Scope(#[from] ScopeError),

    #[error("(expected {expected:?}, found {actual:?})")]
    Mismatch {
        expected: String,
        actual: String,
    }
}

pub struct Parser { pub start: Rc<dyn Instr>, scope: Rc<Scope> }
impl Parser {
    pub fn new(start: Rc<Start>) -> Self {
        Self {
            start: start,
            scope: Rc::new(Scope::new()),
        }
    }

    pub fn parse_prg(&mut self, tokens: &[Token]) -> Result<Rc<dyn Instr>, ParseError> {
        let r = tokens;
        let (_, r) = require(r, TT::KeywordInt)?;
        let (_, r) = require(r, TT::Alias)?;
        let (_, r) = require(r, TT::PuncLeftParen)?;
        let (_, r) = require(r, TT::PuncRightParen)?;
    
        let (_, r) = require(r, TT::PuncLeftBrace)?;
        let (block, r) = self.parse_block(r)?;
        let (_, r) = require(r, TT::PuncRightBrace)?;
    
        if r.is_empty() { Ok(block) } else { Err(ParseError::Mismatch { expected: "empty token stream".to_string(), actual: format!("{:?}", r) }) }
    }
    
    fn parse_block<'a>(&mut self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        self.scope.push_nv();
        let (mut output, mut r) = (None, tokens);
        while let Ok((stmt, _r)) = self.parse_stmt(r) {
            output = Some(stmt);
            r = _r;
        };
        self.scope.push_nv();
        Ok((output.unwrap(), r))
    }
    
    fn parse_stmt<'a>(&self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        match tokens {
            [] => Err(ParseError::Mismatch { expected: "expected: {:?} got an empty token stream".to_string(), actual: "".to_string() }),
            [f, r @ ..] => match f.typ {
                TT::KeywordInt => {
                    let (alias, r) = require(r, TT::Alias)?;
                    let (_, r) = require(r, TT::Equals)?;
                    let (expr, r) = self.parse_expr(r)?;
                    let (_, r) = require(r, TT::PuncSemiColon)?;

                    let _ = self.scope.write(alias.lexeme.to_owned(), expr.clone())?;
                    Ok((expr, r))
                },
                TT::KeywordRet => {
                    let (expr, r) = self.parse_term( r)?;
                    let (_, r) = require(r, TT::PuncSemiColon)?;
                    let retinstr = Return::new(self.start.clone(), expr);
                    Ok((retinstr, r))
                }
                t => Err(ParseError::Mismatch { expected: format!("expected: {:?} got: {:?}", TT::KeywordRet, t), actual: f.lexeme.to_owned() }),
            },
        }
    }

    fn parse_expr<'a>(&self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        self.parse_term(tokens)
    }
    
    fn parse_term<'a>(&self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        let (x, r) = self.parse_factor( tokens)?;
    
        match r {
            [] => panic!(),
            [f, _r @ ..] => match f.typ {
                TT::Plus => {
                    let (y, r) = self.parse_factor(_r)?;
                    Ok((Add::new(x, y).peephole(self.start.clone()), r))
                }
                TT::Minus => {
                    let (y, r) = self.parse_factor( _r)?;
                    Ok((Sub::new(x, y), r))
                }
                _ => {
                    Ok((x, r))
                }
            },
        }
    }
    
    fn parse_factor<'a>(&self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        let (x, r) = self.parse_atom( tokens)?;
    
        match r {
            [] => panic!(),
            [f, _r @ ..] => match f.typ {
                TT::Star => {
                    let (y, r) = self.parse_atom( _r)?;
                    Ok((Mul::new(x, y), r))
                }
                TT::Slash => {
                    let (y, r) = self.parse_atom( _r)?;
                    Ok((Div::new(x, y), r))
                }
                _ => Ok((x, r)),
            },
        }
    }

    fn parse_atom<'a>(&self, tokens: &'a [Token]) -> Result<(Rc<dyn Instr>, &'a [Token]), ParseError> {
        match tokens {
            [] => Err(ParseError::Mismatch { expected: "expected: {:?} got an empty token stream".to_string(), actual: "".to_string() }),
            [f, r @ ..] => match f.typ {
                TT::LiteralInt => {
                    let constantinstr = Int::new(
                        self.start.clone(),
                        TypeKind::Int(f.lexeme.parse().unwrap()),
                    );

                    Ok((constantinstr, r))
                }
                TT::Alias => {
                    let expr = self.scope.read(f.lexeme.to_owned())?;
                    Ok((expr,r))
                },
                t => Err(ParseError::Mismatch { expected: format!("expected: {:?} got: {:?}", TT::LiteralInt, t), actual: f.lexeme.to_owned() }),
            },
        }
    }
}

fn require(tokens: &[Token], tt: TT) -> Result<(&Token, &[Token]), ParseError> {
    match tokens {
        [] => Err(ParseError::Mismatch { expected: format!("expected: {:?} got: {:?}", tt, tokens), actual: "".to_string() }),
        [f, r @ ..] => {
            if f.typ == tt {
                Ok((f, r))
            } else {
                Err(ParseError::Mismatch { expected: format!("expected: {:?} got: {:?}", tt, f), actual: f.lexeme.to_owned() })
            }
        }
    }
}

#[cfg(test)]
mod test_arith {
    use crate::{lexer, rep::ctl::Start};
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/arith";

    #[test]
    fn lit() {
        let chars = fs::read(format!("{TEST_DIR}/lit.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();
    
        let tokens = lexer::lex(&chars).unwrap();
        let mut parser = super::Parser::new(Start::new());
        let graph = parser.parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r###"
        Return {
            id: 3,
            typ: Bot,
            inputs: RefCell {
                value: [
                    Start {
                        id: 1,
                        typ: Bot,
                        inputs: RefCell {
                            value: [],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                                (Weak),
                            ],
                        },
                    },
                    Int {
                        _id: 2,
                        typ: Int(
                            8,
                        ),
                        inputs: RefCell {
                            value: [
                                Start {
                                    id: 1,
                                    typ: Bot,
                                    inputs: RefCell {
                                        value: [],
                                    },
                                    outputs: RefCell {
                                        value: [
                                            (Weak),
                                            (Weak),
                                        ],
                                    },
                                },
                            ],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                            ],
                        },
                    },
                ],
            },
            outputs: RefCell {
                value: [],
            },
        }
        "###);
    }

    #[test]
    fn add() {
        let chars = fs::read(format!("{TEST_DIR}/add.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let mut parser = super::Parser::new(Start::new());
        let tokens = lexer::lex(&chars).unwrap();
        let graph = parser.parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r###"
        Return {
            id: 6,
            typ: Bot,
            inputs: RefCell {
                value: [
                    Start {
                        id: 1,
                        typ: Bot,
                        inputs: RefCell {
                            value: [],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                            ],
                        },
                    },
                    Int {
                        _id: 5,
                        typ: Int(
                            19,
                        ),
                        inputs: RefCell {
                            value: [
                                Start {
                                    id: 1,
                                    typ: Bot,
                                    inputs: RefCell {
                                        value: [],
                                    },
                                    outputs: RefCell {
                                        value: [
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                        ],
                                    },
                                },
                            ],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                            ],
                        },
                    },
                ],
            },
            outputs: RefCell {
                value: [],
            },
        }
        "###);
    }
}

#[cfg(test)]
mod test_bindings {
    use crate::{lexer, rep::ctl::Start};
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";

    #[test]
    fn assignment() {
        let chars = fs::read(format!("{TEST_DIR}/asnmt_multi_expr_var.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

            let mut parser = super::Parser::new(Start::new());
            let tokens = lexer::lex(&chars).unwrap();
            let graph = parser.parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r###"
        Return {
            id: 12,
            typ: Bot,
            inputs: RefCell {
                value: [
                    Start {
                        id: 1,
                        typ: Bot,
                        inputs: RefCell {
                            value: [],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                                (Weak),
                            ],
                        },
                    },
                    Int {
                        _id: 11,
                        typ: Int(
                            38,
                        ),
                        inputs: RefCell {
                            value: [
                                Start {
                                    id: 1,
                                    typ: Bot,
                                    inputs: RefCell {
                                        value: [],
                                    },
                                    outputs: RefCell {
                                        value: [
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                            (Weak),
                                        ],
                                    },
                                },
                            ],
                        },
                        outputs: RefCell {
                            value: [
                                (Weak),
                                (Weak),
                            ],
                        },
                    },
                ],
            },
            outputs: RefCell {
                value: [],
            },
        }
        "###);
    }
}
