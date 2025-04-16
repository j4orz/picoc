use crate::{
    rep::{ctl::{Return, Start}, data::{Add, Constant, Div, Mul, Sub}, Instr, Type},
    lexer::{Token, TT},
    optimizer::peephole,
};
use std::{io, sync::{Arc, LazyLock, Mutex}};

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

pub static START: Start = Start { id: 0, typ: Type::Bot };
static SCOPE: LazyLock<Mutex<ScopeFields>> = LazyLock::new(|| Mutex::new(ScopeFields::new()));

pub fn parse_prg(tokens: &[Token]) -> Result<Box<dyn Instr>, io::Error> {
    let r = tokens;
    let (_, r) = require(r, TT::KeywordInt)?;
    let (_, r) = require(r, TT::Alias)?;
    let (_, r) = require(r, TT::PuncLeftParen)?;
    let (_, r) = require(r, TT::PuncRightParen)?;

    let (_, r) = require(r, TT::PuncLeftBrace)?;
    let (stmt, r) = parse_stmt(r)?;
    let (_, r) = require(r, TT::PuncRightBrace)?;

    if r.is_empty() { Ok(stmt) } else { Err(io::Error::new(io::ErrorKind::Other,format!("expected empty token stream, {:?}", r)))}
}

fn parse_block(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    SCOPE.lock().unwrap().push_nv();
    let (mut foo, mut r) = (None, tokens);
    while let Ok((bar, _r)) = parse_stmt(r) {
        foo = Some(bar);
        r = _r;
    };
    SCOPE.lock().unwrap().pop_nv();
    Ok((foo.unwrap(), r))
}

fn parse_stmt(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::KeywordInt => {
                let (alias, r) = require(r, TT::Alias)?;
                let (_, r) = require(r, TT::Equals)?;
                let (expr, r) = parse_expr(r)?;
                let (_, r) = require(r, TT::PuncSemiColon)?;
                
                // Ok((SStmt::Asnmt(a), r))
                todo!()
            },
            TT::KeywordRet => {
                let (expr, r) = parse_term( r)?;
                let (_, r) = require(r, TT::PuncSemiColon)?;
                let retinstr = Box::new(Return::new(Arc::new(START.clone()), expr));
                Ok((retinstr, r))
            }
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("expected: {:?} got: {:?}", TT::KeywordRet, t),
            )),
        },
    }
}

fn parse_expr(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    parse_term(tokens)
}

fn parse_term(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    let (x, r) = parse_factor( tokens)?;

    match r {
        [] => panic!(),
        [f, _r @ ..] => match f.typ {
            TT::Plus => {
                let (y, r) = parse_factor(_r)?;
                Ok((peephole(Box::new(Add::new(x, y))), r))
            }
            TT::Minus => {
                let (y, r) = parse_factor( _r)?;
                Ok((Box::new(Sub::new(x, y)), r))
            }
            t => {
                println!("moose {:?}", r);
                Ok((x, r))
            }
        },
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    let (x, r) = parse_atom( tokens)?;

    match r {
        [] => panic!(),
        [f, _r @ ..] => match f.typ {
            TT::Star => {
                let (y, r) = parse_atom( _r)?;
                Ok((Box::new(Mul::new(x, y)), r))
            }
            TT::Slash => {
                let (y, r) = parse_atom( _r)?;
                Ok((Box::new(Div::new(x, y)), r))
            }
            _ => Ok((x, r)),
        },
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(Box<dyn Instr>, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::LiteralInt => {
                let constantinstr = Constant::new(
                    Arc::new(START.clone()),
                    Type::Int(f.lexeme.parse().unwrap()),
                );
                Ok((Box::new(constantinstr), r))
            }
            // TT:Alias ...
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("expected: {:?} got: {:?}", TT::LiteralInt, t),
            )),
        },
    }
}

fn require(tokens: &[Token], tt: TT) -> Result<(&Token, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("expected: {:?} got: {:?}", tt, tokens),
        )),
        [f, r @ ..] => {
            if f.typ == tt {
                Ok((f, r))
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("expected: {:?} got: {:?}", tt, f),
                ))
            }
        }
    }
}

#[cfg(test)]
mod test_arith {
    use crate::lexer;
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
        let graph = super::parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r###"
        Return(
            ReturnFields {
                id: 2,
                typ: Bot,
                ud: [
                    Start(
                        StartFields {
                            id: 0,
                            typ: Bot,
                        },
                    ),
                    Constant(
                        ConstantFields {
                            id: 1,
                            typ: Int(
                                8,
                            ),
                            ud: [
                                Start(
                                    StartFields {
                                        id: 0,
                                        typ: Bot,
                                    },
                                ),
                            ],
                            du: [],
                        },
                    ),
                ],
                du: [],
                ctrl: Start(
                    StartFields {
                        id: 0,
                        typ: Bot,
                    },
                ),
                data: Constant(
                    ConstantFields {
                        id: 1,
                        typ: Int(
                            8,
                        ),
                        ud: [
                            Start(
                                StartFields {
                                    id: 0,
                                    typ: Bot,
                                },
                            ),
                        ],
                        du: [],
                    },
                ),
            },
        )
        "###);
    }

    #[test]
    fn add() {
        let chars = fs::read(format!("{TEST_DIR}/add.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let graph = super::parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r###"
        Return(
            ReturnFields {
                id: 5,
                typ: Bot,
                ud: [
                    Start(
                        StartFields {
                            id: 0,
                            typ: Bot,
                        },
                    ),
                    Constant(
                        ConstantFields {
                            id: 4,
                            typ: Int(
                                19,
                            ),
                            ud: [
                                Start(
                                    StartFields {
                                        id: 0,
                                        typ: Bot,
                                    },
                                ),
                            ],
                            du: [],
                        },
                    ),
                ],
                du: [],
                ctrl: Start(
                    StartFields {
                        id: 0,
                        typ: Bot,
                    },
                ),
                data: Constant(
                    ConstantFields {
                        id: 4,
                        typ: Int(
                            19,
                        ),
                        ud: [
                            Start(
                                StartFields {
                                    id: 0,
                                    typ: Bot,
                                },
                            ),
                        ],
                        du: [],
                    },
                ),
            },
        )
        "###);
    }
}


#[cfg(test)]
mod test_bindings {
    use crate::lexer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";

    #[test]
    fn assignment() {
        let chars = fs::read(format!("{TEST_DIR}/assignment.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let graph = super::parse_prg(&tokens).unwrap();
        insta::assert_debug_snapshot!(graph, @r"");
    }
}