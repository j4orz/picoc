use crate::{
    lexer::{Token, TT},
    OldType, SBinOp, SDef, SExpr, SFuncDef, SPrg, SRelOp, SStmt, SVarDef,
};
use std::io;
use std::num::ParseIntError;

fn eat(tokens: &[Token], tt: TT) -> Result<(&Token, &[Token]), io::Error> {
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

pub fn parse_prg(tokens: &[Token]) -> Result<SPrg, io::Error> {
    let (mut fds, mut r) = (vec![], tokens);
    while let Ok((fd, _r)) = parse_funcdef(r) {
        fds.push(fd);
        r = _r;
    }

    Ok(fds.into_iter().map(SDef::FuncDef).collect())
}

fn parse_funcdef(tokens: &[Token]) -> Result<(SFuncDef, &[Token]), io::Error> {
    let (_, r) = eat(tokens, TT::KeywordInt)?;
    let (alias, r) = eat(r, TT::Alias)?;
    let (_, r) = eat(r, TT::PuncLeftParen)?;

    let (mut fps, mut r) = (vec![], r);
    while let Ok((_fp_type, _r)) = eat(r, TT::KeywordInt) {
        let (alias, _r) = eat(_r, TT::Alias)?;
        fps.push((alias.lexeme.to_owned(), OldType::Int));

        if let TT::PuncComma = _r[0].typ {
            r = &_r[1..];
        } else {
            r = _r;
        }
    }
    let (_, r) = eat(r, TT::PuncRightParen)?;
    let (_, r) = eat(r, TT::PuncLeftBrace)?;

    let (mut stmts, mut r) = (vec![], r);
    while let Ok((s, _r)) = parse_stmt(r) {
        stmts.push(s);
        r = _r;
    }
    let (_, r) = eat(r, TT::PuncRightBrace)?;

    Ok((SFuncDef { alias: alias.lexeme.to_string(), typ: OldType::Int, fps, body: stmts }, r))
}

fn parse_vardef(tokens: &[Token]) -> Result<(SVarDef, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            // for now int is parsed with vardef
            TT::KeywordInt => {
                let (alias, r) = eat(r, TT::Alias)?;
                let (_, r) = eat(r, TT::Equals)?;
                let (expr, r) = parse_expr(r)?;

                Ok((
                    SVarDef {
                        alias: alias.lexeme.to_owned(),
                        typ: OldType::Int,
                        expr: Box::new(expr),
                    },
                    r,
                ))
            }
            TT::Alias => match r {
                [] => todo!(),
                [s, t, _r @ ..] => match (s.typ, t.typ) {
                    // (TT::Plus, TT::Equals) => {
                    //     let (expr, r) = parse_rel_expr(r)?;
                    //     Ok((
                    //         VarDef::UpdateBind {
                    //             alias: f.lexeme.parse().unwrap(),
                    //             op: BinOp::Add,
                    //             expr: Box::new(expr),
                    //         },
                    //         r,
                    //     ))
                    // }
                    // (TT::Plus, TT::Plus) => Ok((
                    //     VarDef::UpdateBind {
                    //         alias: f.lexeme.parse().unwrap(),
                    //         op: BinOp::Add,
                    //         expr: Box::new(Expr::Int(1)),
                    //     },
                    //     r,
                    // )),
                    // (TT::Minus, TT::Equals) => {
                    //     let (expr, r) = parse_rel_expr(r)?;

                    //     Ok((
                    //         VarDef::UpdateBind {
                    //             alias: f.lexeme.parse().unwrap(),
                    //             op: BinOp::Sub,
                    //             expr: Box::new(expr),
                    //         },
                    //         r,
                    //     ))
                    // }
                    // (TT::Minus, TT::Minus) => Ok((
                    //     VarDef::UpdateBind {
                    //         alias: f.lexeme.parse().unwrap(),
                    //         op: BinOp::Sub,
                    //         expr: Box::new(Expr::Int(1)),
                    //     },
                    //     r,
                    // )),
                    // (TT::Star, TT::Equals) => {
                    //     let (expr, r) = parse_rel_expr(r)?;

                    //     Ok((
                    //         VarDef::UpdateBind {
                    //             alias: f.lexeme.parse().unwrap(),
                    //             op: BinOp::Mult,
                    //             expr: Box::new(expr),
                    //         },
                    //         r,
                    //     ))
                    // }
                    // (TT::Slash, TT::Equals) => {
                    //     let (expr, r) = parse_rel_expr(r)?;

                    //     Ok((
                    //         VarDef::UpdateBind {
                    //             alias: f.lexeme.parse().unwrap(),
                    //             op: BinOp::Div,
                    //             expr: Box::new(expr),
                    //         },
                    //         r,
                    //     ))
                    // }
                    _t => {
                        todo!()
                    }
                },
                _t => todo!(),
            },
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
    }
}

fn parse_stmt(tokens: &[Token]) -> Result<(SStmt, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TT::KeywordInt => {
                // todo: | TT:KeywordAlias{++, --, -=}, etc.
                let (a, r) = parse_vardef(tokens)?;
                let (_, r) = eat(r, TT::PuncSemiColon)?;

                Ok((SStmt::Asnmt(a), r))
            }
            TT::KeywordRet => {
                let (expr, r) = parse_rel(r)?;
                let (_, r) = eat(r, TT::PuncSemiColon)?;
                Ok((SStmt::Return(expr), r))
            }
            TT::KeywordIf => {
                let (_, r) = eat(r, TT::PuncLeftParen)?;
                let (cond, r) = parse_rel(r)?;
                let (_, r) = eat(r, TT::PuncRightParen)?;
                let (_, r) = eat(r, TT::PuncLeftBrace)?;
                let (then, r) = parse_stmt(r)?;
                let (_, r) = eat(r, TT::PuncRightBrace)?;
                let (els, r) = if let TT::KeywordEls = r[0].typ {
                    let (_, r) = eat(r, TT::KeywordEls)?;
                    let (_, r) = eat(r, TT::PuncLeftBrace)?;
                    let (els, r) = parse_stmt(r)?;
                    let (_, r) = eat(r, TT::PuncRightBrace)?;
                    (Some(Box::new(els)), r)
                } else {
                    (None, r)
                };

                Ok((SStmt::IfEls { cond: Box::new(cond), then: Box::new(then), els }, r))
            }
            TT::KeywordWhile => {
                let (_, r) = eat(r, TT::PuncLeftParen)?;
                let (cond, r) = parse_rel(r)?;
                let (_, r) = eat(r, TT::PuncRightParen)?;
                let (_, r) = eat(r, TT::PuncLeftBrace)?;
                let (body, r) = parse_stmt(r)?;
                let (_, r) = eat(r, TT::PuncRightBrace)?;

                Ok((SStmt::While { cond: Box::new(cond), body: Box::new(body) }, r))
            }
            // TT::KeywordFor => {
            //     let (_, r) = mtch(r, TT::PuncLeftParen)?;
            //     let (asnmt, r) = parse_asmt(r)?;
            //     let (_, r) = mtch(r, TT::PuncSemiColon)?;
            //     let (cond, r) = parse_rel_expr(r)?;
            //     let (_, r) = mtch(r, TT::PuncSemiColon)?;
            //     let (update, r) = parse_asmt(r)?;
            //     let (_, r) = mtch(r, TT::PuncRightParen)?;
            //     let (_, r) = mtch(r, TT::PuncLeftBrace)?;

            //     let mut body = vec![];
            //     let mut r0 = r;
            //     while let Ok((s, r1)) = parse_stmt(r0) {
            //         body.push(s);
            //         r0 = r1;
            //     }
            //     let (_, r) = mtch(r0, TT::PuncRightBrace)?;

            //     Ok((
            //         Stmt::For {
            //             asnmt: Box::new(asnmt),
            //             cond: Box::new(cond),
            //             update: Box::new(update),
            //             body,
            //         },
            //         r,
            //     ))
            // }
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
    }
}

fn parse_expr(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    parse_rel(tokens)
}

fn parse_rel(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    let (left, r) = parse_term(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;

            while let Ok((op, _r)) = parse_rel_op(r) {
                let (right, _r) = parse_term(_r)?;

                cur_node = SExpr::RelE { op, l: Box::new(cur_node), r: Box::new(right) };

                r = _r;
            }

            Ok((cur_node, r))
        }
    }
}
fn parse_rel_op(tokens: &[Token]) -> Result<(SRelOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TT::LeftAngleBracket => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Equals => Ok((SRelOp::LtEq, r)),
                    _ => Ok((SRelOp::Lt, &tokens[1..])), // include s
                },
            },
            TT::RightAngleBracket => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Equals => Ok((SRelOp::GtEq, r)),
                    _ => Ok((SRelOp::Gt, &tokens[1..])), // include s
                },
            },
            TT::Equals => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Equals => Ok((SRelOp::Eq, r)),
                    t => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("token not recognizable {:?}", t),
                    )),
                },
            },
            TT::Bang => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Equals => Ok((SRelOp::Neq, r)),
                    t => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("token not recognizable {:?}", t),
                    )),
                },
            },
            TT::Amp => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Amp => Ok((SRelOp::And, r)),
                    t => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("token not recognizable {:?}", t),
                    )),
                },
            },
            TT::Bar => match r {
                [] => todo!(),
                [s, r @ ..] => match s.typ {
                    TT::Bar => Ok((SRelOp::Or, r)),
                    t => Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("token not recognizable {:?}", t),
                    )),
                },
            },
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
    }
}

fn parse_term(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    let (left, r) = parse_factor(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;

            while let Ok((op, _r)) = parse_term_op(r) {
                let (right, _r) = parse_factor(_r)?;

                cur_node = SExpr::BinE { op, l: Box::new(cur_node), r: Box::new(right) };

                r = _r;
            }

            Ok((cur_node, r))
        }
    }
}
fn parse_term_op(tokens: &[Token]) -> Result<(SBinOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TT::Plus => Ok((SBinOp::Add, r)),
            TT::Minus => Ok((SBinOp::Sub, r)),
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
    }
}

fn parse_factor(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    let (left, r) = parse_funcapp(tokens)?;

    match r {
        [] => Ok((left, r)),
        r => {
            let mut cur_node = left;
            let mut r = r;
            while let Ok((op, _r)) = parse_factor_op(r) {
                let (right, _r) = parse_atom(_r)?;

                cur_node = SExpr::BinE { op, l: Box::new(cur_node), r: Box::new(right) };

                r = _r;
            }

            Ok((cur_node, r))
        }
    }
}

fn parse_factor_op(tokens: &[Token]) -> Result<(SBinOp, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TT::Star => Ok((SBinOp::Mult, r)),
            TT::Slash => Ok((SBinOp::Div, r)),
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
    }
}

fn parse_funcapp(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    let (left, r0) = parse_atom(tokens)?;

    match r0 {
        [] => Ok((left, r0)),
        [f, r @ ..] => {
            if let TT::PuncLeftParen = f.typ {
                let (mut aps, mut r) = (vec![], r);
                while let Ok((ap, _r)) = parse_expr(r) {
                    aps.push(ap);

                    if let TT::PuncComma = _r[0].typ {
                        r = &_r[1..];
                    } else {
                        r = _r;
                    }
                }
                let (_, r) = eat(r, TT::PuncRightParen)?;

                match left {
                    SExpr::VarApp(alias) => Ok((SExpr::FuncApp { alias, aps }, r)),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "expected alias".to_string())),
                }
            } else {
                Ok((left, r0))
            }
        }
    }
}

fn parse_atom(tokens: &[Token]) -> Result<(SExpr, &[Token]), io::Error> {
    match tokens {
        [] => todo!(),
        [f, r @ ..] => match f.typ {
            TT::Alias => Ok((SExpr::VarApp(f.lexeme.to_owned()), r)),
            TT::LiteralInt => Ok((
                SExpr::Int(f.lexeme.parse().map_err(|e: ParseIntError| {
                    io::Error::new(io::ErrorKind::Other, e.to_string())
                })?),
                r,
            )),
            TT::KeywordTrue => Ok((SExpr::Bool(true), r)),
            TT::KeywordFalse => Ok((SExpr::Bool(false), r)),
            t => {
                Err(io::Error::new(io::ErrorKind::Other, format!("token not recognizable {:?}", t)))
            }
        },
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
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  Int: 8
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
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      Int: 9
                    r:
                      Int: 10
        "###);
    }

    #[test]
    fn add_multi() {
        let chars = fs::read(format!("{TEST_DIR}/add_multi.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      BinE:
                        op: Add
                        l:
                          Int: 9
                        r:
                          Int: 10
                    r:
                      Int: 11
        "###);
    }

    #[test]
    fn sub() {
        let chars = fs::read(format!("{TEST_DIR}/sub.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Sub
                    l:
                      Int: 88
                    r:
                      Int: 32
        "###);
    }

    #[test]
    fn mult() {
        let chars = fs::read(format!("{TEST_DIR}/mult.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Mult
                    l:
                      Int: 9
                    r:
                      Int: 10
        "###);
    }

    #[test]
    fn div() {
        let chars = fs::read(format!("{TEST_DIR}/div.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Div
                    l:
                      Int: 100
                    r:
                      Int: 9
        "###);
    }

    #[test]
    fn add_associative() {
        let chars = fs::read(format!("{TEST_DIR}/add_associative.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      BinE:
                        op: Add
                        l:
                          Int: 9
                        r:
                          Int: 10
                    r:
                      Int: 11
        "###);
    }

    #[test]
    fn sub_associative() {
        let chars = fs::read(format!("{TEST_DIR}/sub_associative.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Sub
                    l:
                      BinE:
                        op: Sub
                        l:
                          Int: 30
                        r:
                          Int: 9
                    r:
                      Int: 10
        "###);
    }

    #[test]
    fn mult_add_precedence() {
        let chars = fs::read(format!("{TEST_DIR}/mult_add_precedence.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      BinE:
                        op: Mult
                        l:
                          Int: 9
                        r:
                          Int: 10
                    r:
                      Int: 11
        "###);
    }

    #[test]
    fn mult_add_precedence_multi() {
        let chars = fs::read(format!("{TEST_DIR}/mult_add_precedence_multi.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      BinE:
                        op: Mult
                        l:
                          Int: 9
                        r:
                          Int: 10
                    r:
                      BinE:
                        op: Mult
                        l:
                          Int: 11
                        r:
                          Int: 12
        "###);
    }
}

#[cfg(test)]
mod test_control_c0 {
    use crate::lexer;
    use std::fs;
    const TEST_DIR: &str = "tests/fixtures/snap/statics-c0/control";

    #[test]
    fn ifels() {
        let chars = fs::read(format!("{TEST_DIR}/if.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - IfEls:
                  cond:
                    Bool: true
                  then:
                    Return:
                      Int: 9
                  els:
                    Return:
                      Int: 10
        "###);
    }

    #[test]
    fn ifnoels() {
        let chars = fs::read(format!("{TEST_DIR}/if4.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - IfEls:
                  cond:
                    Bool: true
                  then:
                    Asnmt:
                      alias: x
                      typ: Int
                      expr:
                        Int: 8
                  els: ~
              - IfEls:
                  cond:
                    Bool: false
                  then:
                    Return:
                      Int: 9
                  els: ~
              - IfEls:
                  cond:
                    Bool: false
                  then:
                    Return:
                      Int: 10
                  els: ~
              - IfEls:
                  cond:
                    Bool: false
                  then:
                    Return:
                      Int: 11
                  els: ~
              - IfEls:
                  cond:
                    Bool: true
                  then:
                    Return:
                      Int: 12
                  els: ~
        "###);
    }
}

#[cfg(test)]
mod test_control_c89 {
    use crate::lexer;
    use std::fs;
    const TEST_DIR: &str = "tests/fixtures/snap/shared/control";

    #[test]
    fn eq() {
        let chars = fs::read(format!("{TEST_DIR}/eq_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: Eq
                    l:
                      Int: 9
                    r:
                      Int: 9
        "###);
    }

    #[test]
    fn neq() {
        let chars = fs::read(format!("{TEST_DIR}/neq_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: Neq
                    l:
                      Int: 9
                    r:
                      Int: 10
        "###);
    }

    #[test]
    fn and() {
        let chars = fs::read(format!("{TEST_DIR}/and_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: And
                    l:
                      Int: 1
                    r:
                      Int: 1
        "###);
    }

    #[test]
    fn or() {
        let chars = fs::read(format!("{TEST_DIR}/or_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: Or
                    l:
                      Int: 1
                    r:
                      Int: 1
        "###);
    }

    #[test]
    fn lt() {
        let chars = fs::read(format!("{TEST_DIR}/lt_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: Lt
                    l:
                      Int: 9
                    r:
                      Int: 10
        "###);
    }

    #[test]
    fn gt() {
        let chars = fs::read(format!("{TEST_DIR}/gt_true.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  RelE:
                    op: Gt
                    l:
                      Int: 10
                    r:
                      Int: 9
        "###);
    }

    #[test]
    fn ifels_then() {
        let chars = fs::read(format!("{TEST_DIR}/ifels_then.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - IfEls:
                  cond:
                    Int: 1
                  then:
                    Return:
                      Int: 9
                  els:
                    Return:
                      Int: 10
        "###);
    }

    // #[test]
    // fn for_loop() {
    //     let chars = fs::read(format!("{TEST_DIR}/for.c"))
    //         .expect("file dne")
    //         .iter()
    //         .map(|b| *b as char)
    //         .collect::<Vec<_>>();

    //     let tokens = lexer::lex(&chars).unwrap();
    //     let tree = super::parse_prg(&tokens).unwrap();
    //     insta::assert_yaml_snapshot!(tree, @r###"
    //     ---
    //     main_function:
    //       stmts:
    //         - Asnmt:
    //             CreateBind:
    //               id: n
    //               expr:
    //                 Int: 0
    //         - For:
    //             asnmt:
    //               CreateBind:
    //                 id: i
    //                 expr:
    //                   Int: 0
    //             cond:
    //               RelE:
    //                 op: Lt
    //                 l:
    //                   Var: i
    //                 r:
    //                   Int: 10
    //             update:
    //               UpdateBind:
    //                 id: i
    //                 op: Add
    //                 expr:
    //                   Int: 1
    //             body:
    //               - Asnmt:
    //                   UpdateBind:
    //                     id: n
    //                     op: Add
    //                     expr:
    //                       Int: 1
    //               - Asnmt:
    //                   UpdateBind:
    //                     id: n
    //                     op: Add
    //                     expr:
    //                       Int: 1
    //         - Return:
    //             Var: n
    //     "###);
    // }
}

#[cfg(test)]
mod test_bindings {
    use crate::lexer;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";

    #[test]
    fn composition() {
        let chars = fs::read(format!("{TEST_DIR}/composition.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: h
            typ: Int
            fps: []
            body:
              - Return:
                  Int: 11
        - FuncDef:
            alias: g
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      Int: 10
                    r:
                      FuncApp:
                        alias: h
                        aps: []
        - FuncDef:
            alias: f
            typ: Int
            fps: []
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      Int: 9
                    r:
                      FuncApp:
                        alias: g
                        aps: []
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  FuncApp:
                    alias: f
                    aps: []
        "###);
    }

    #[test]
    fn formal_param() {
        let chars = fs::read(format!("{TEST_DIR}/formal_param.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: f
            typ: Int
            fps:
              - - x
                - Int
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      VarApp: x
                    r:
                      Int: 10
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  FuncApp:
                    alias: f
                    aps:
                      - Int: 9
        "###);
    }

    #[test]
    fn formal_param_multi() {
        let chars = fs::read(format!("{TEST_DIR}/formal_param_multi.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: f
            typ: Int
            fps:
              - - x
                - Int
              - - y
                - Int
            body:
              - Return:
                  BinE:
                    op: Add
                    l:
                      VarApp: x
                    r:
                      VarApp: y
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Return:
                  FuncApp:
                    alias: f
                    aps:
                      - Int: 9
                      - Int: 10
        "###);
    }

    #[test]
    fn asnmt() {
        let chars = fs::read(format!("{TEST_DIR}/assignment.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = super::parse_prg(&tokens).unwrap();
        insta::assert_yaml_snapshot!(tree, @r###"
        ---
        - FuncDef:
            alias: main
            typ: Int
            fps: []
            body:
              - Asnmt:
                  alias: x
                  typ: Int
                  expr:
                    Int: 9
              - Return:
                  VarApp: x
        "###);
    }

    // #[test]
    // fn asnmt_update() {
    //     let chars = fs::read(format!("{TEST_DIR}/asnmt_update.c"))
    //         .expect("file dne")
    //         .iter()
    //         .map(|b| *b as char)
    //         .collect::<Vec<_>>();

    //     let tokens = lexer::lex(&chars).unwrap();
    //     let tree = super::parse_prg(&tokens).unwrap();
    //     insta::assert_yaml_snapshot!(tree, @r###"
    //     ---
    //     main_function:
    //       stmts:
    //         - Asnmt:
    //             CreateBind:
    //               id: n
    //               expr:
    //                 Int: 0
    //         - Asnmt:
    //             UpdateBind:
    //               id: n
    //               op: Add
    //               expr:
    //                 Int: 10
    //         - Return:
    //             Var: n
    //     "###);
    // }
}
