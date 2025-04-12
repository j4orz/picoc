use crate::{
    lexer::{Token, TT},
    ConstantFields, Instr, ReturnFields, StartFields,
};
use std::io;

pub fn parse_prg(tokens: &[Token]) -> Result<Instr, io::Error> {
    let startinstr = Instr::Start(StartFields::new());
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

fn parse_stmt(start: Instr, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::KeywordRet => {
                let (expr, r) = parse_expr(start.clone(), r)?;
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

fn parse_expr(start: Instr, tokens: &[Token]) -> Result<(Instr, &[Token]), io::Error> {
    match tokens {
        [] => Err(io::Error::new(io::ErrorKind::Other, "expected: {:?} got an empty token stream")),
        [f, r @ ..] => match f.typ {
            TT::LiteralInt => {
                let constantinstr =
                    Instr::Constant(ConstantFields::new(start, f.lexeme.parse().unwrap()));
                Ok((constantinstr, r))
            }
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
        insta::assert_debug_snapshot!(graph, @"");
    }
}
