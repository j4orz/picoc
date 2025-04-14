use crate::{
    lexer::{Token, TT},
    AddFields, ConstantFields, DivFields, Instr, MulFields, ReturnFields, StartFields, SubFields,
    Type,
};
use std::{io, rc::Rc};

pub fn parse_prg(tokens: &[Token]) -> Result<Instr, io::Error> {
    let startinstr = Rc::new(Instr::Start(StartFields::new()));
    let r = tokens;
    let (_, r) = mtch(r, TT::KeywordInt)?;
    let (_, r) = mtch(r, TT::Alias)?;
    let (_, r) = mtch(r, TT::PuncLeftParen)?;
    let (_, r) = mtch(r, TT::PuncRightParen)?;

    let (_, r) = mtch(r, TT::PuncLeftBrace)?;
    let (stmt, r) = parse_stmt(startinstr, r)?;
    let (_, r) = mtch(r, TT::PuncRightBrace)?;

    if r.is_empty() {
        Ok(stmt)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("expected empty token stream, got {:?}", r),
        ))
    }
}

fn parse_stmt(start: Rc<Instr>, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::KeywordRet => {
                let (expr, r) = parse_term(start.clone(), r)?;
                let (_, r) = mtch(r, TT::PuncSemiColon)?;
                let retinstr = Instr::Return(ReturnFields::new(start, expr));
                Ok((retinstr, r))
            }
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("expected: {:?} got: {:?}", TT::KeywordRet, t),
            )),
        },
    }
}

fn parse_term(start: Rc<Instr>, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    let (x, r) = parse_factor(start.clone(), tokens)?;

    match r {
        [] => panic!(),
        [f, _r @ ..] => match f.typ {
            TT::Plus => {
                let (y, r) = parse_factor(start, _r)?;
                Ok((Instr::Add(AddFields::new(x, y)), r))
            }
            TT::Minus => {
                let (y, r) = parse_factor(start, _r)?;
                Ok((Instr::Sub(SubFields::new(x, y)), r))
            }
            t => {
                println!("moose {:?}", r);
                Ok((x, _r))
            }
        },
    }
}

fn parse_factor(start: Rc<Instr>, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    let (x, r) = parse_atom(start.clone(), tokens)?;

    match r {
        [] => panic!(),
        [f, _r @ ..] => match f.typ {
            TT::Star => {
                let (y, r) = parse_atom(start, _r)?;
                Ok((Instr::Mul(MulFields::new(x, y)), r))
            }
            TT::Slash => {
                let (y, r) = parse_atom(start, _r)?;
                Ok((Instr::Div(DivFields::new(x, y)), r))
            }
            _ => Ok((x, r)),
        },
    }
}

fn parse_atom(start: Rc<Instr>, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::LiteralInt => {
                let constantinstr = Instr::Constant(ConstantFields::new(
                    start,
                    Type::Int(f.lexeme.parse().unwrap()),
                ));
                Ok((constantinstr, r))
            }
            // TT:Alias ...
            t => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("expected: {:?} got: {:?}", TT::LiteralInt, t),
            )),
        },
    }
}

fn mtch(tokens: &[Token], tt: TT) -> Result<(&Token, &[Token]), io::Error> {
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
                id: 3,
                ud: [
                    Start(
                        StartFields {
                            id: 1,
                        },
                    ),
                    Constant(
                        ConstantFields {
                            id: 2,
                            ud: [
                                Start(
                                    StartFields {
                                        id: 1,
                                    },
                                ),
                            ],
                            du: [],
                            val: 8,
                        },
                    ),
                ],
                du: [],
                ctrl: Start(
                    StartFields {
                        id: 1,
                    },
                ),
                data: Constant(
                    ConstantFields {
                        id: 2,
                        ud: [
                            Start(
                                StartFields {
                                    id: 1,
                                },
                            ),
                        ],
                        du: [],
                        val: 8,
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
                            id: 1,
                            typ: Bot,
                        },
                    ),
                    Add(
                        AddFields {
                            id: 4,
                            typ: Bot,
                            ud: [
                                Constant(
                                    ConstantFields {
                                        id: 2,
                                        typ: Int(
                                            9,
                                        ),
                                        ud: [
                                            Start(
                                                StartFields {
                                                    id: 1,
                                                    typ: Bot,
                                                },
                                            ),
                                        ],
                                        du: [],
                                    },
                                ),
                                Constant(
                                    ConstantFields {
                                        id: 3,
                                        typ: Int(
                                            10,
                                        ),
                                        ud: [
                                            Start(
                                                StartFields {
                                                    id: 1,
                                                    typ: Bot,
                                                },
                                            ),
                                        ],
                                        du: [],
                                    },
                                ),
                            ],
                            du: [],
                            x: Constant(
                                ConstantFields {
                                    id: 2,
                                    typ: Int(
                                        9,
                                    ),
                                    ud: [
                                        Start(
                                            StartFields {
                                                id: 1,
                                                typ: Bot,
                                            },
                                        ),
                                    ],
                                    du: [],
                                },
                            ),
                            y: Constant(
                                ConstantFields {
                                    id: 3,
                                    typ: Int(
                                        10,
                                    ),
                                    ud: [
                                        Start(
                                            StartFields {
                                                id: 1,
                                                typ: Bot,
                                            },
                                        ),
                                    ],
                                    du: [],
                                },
                            ),
                        },
                    ),
                ],
                du: [],
                ctrl: Start(
                    StartFields {
                        id: 1,
                        typ: Bot,
                    },
                ),
                data: Add(
                    AddFields {
                        id: 4,
                        typ: Bot,
                        ud: [
                            Constant(
                                ConstantFields {
                                    id: 2,
                                    typ: Int(
                                        9,
                                    ),
                                    ud: [
                                        Start(
                                            StartFields {
                                                id: 1,
                                                typ: Bot,
                                            },
                                        ),
                                    ],
                                    du: [],
                                },
                            ),
                            Constant(
                                ConstantFields {
                                    id: 3,
                                    typ: Int(
                                        10,
                                    ),
                                    ud: [
                                        Start(
                                            StartFields {
                                                id: 1,
                                                typ: Bot,
                                            },
                                        ),
                                    ],
                                    du: [],
                                },
                            ),
                        ],
                        du: [],
                        x: Constant(
                            ConstantFields {
                                id: 2,
                                typ: Int(
                                    9,
                                ),
                                ud: [
                                    Start(
                                        StartFields {
                                            id: 1,
                                            typ: Bot,
                                        },
                                    ),
                                ],
                                du: [],
                            },
                        ),
                        y: Constant(
                            ConstantFields {
                                id: 3,
                                typ: Int(
                                    10,
                                ),
                                ud: [
                                    Start(
                                        StartFields {
                                            id: 1,
                                            typ: Bot,
                                        },
                                    ),
                                ],
                                du: [],
                            },
                        ),
                    },
                ),
            },
        )
        "###);
    }
}
