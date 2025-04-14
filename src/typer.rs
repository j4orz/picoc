use crate::{LambdaType, OldType, SBinOp, SDef, SExpr, SFuncDef, SPrg, SStmt, Tnv};
use std::collections::HashMap;
use std::io;

pub fn type_prg(prg: &SPrg) -> Result<OldType, io::Error> {
    let mut tnv = Tnv { fnv: HashMap::new(), vnv: HashMap::new() };

    let _ = prg
        .iter()
        .map(|def| match def {
            SDef::FuncDef(fd) => {
                let ltnv = HashMap::new();
                let type_check = type_func(fd, &tnv, ltnv).map(|t| {
                    tnv.fnv.insert(
                        fd.alias.clone(),
                        LambdaType { fp: fd.fps.iter().map(|(_, t)| t.clone()).collect(), body: t },
                    );
                });
                type_check
            }
            SDef::VarDef(_vd) => todo!(),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(tnv
        .fnv
        .get("main")
        .ok_or(io::Error::new(io::ErrorKind::Other, "main function not found"))?
        .body
        .clone())
}

pub fn type_func(
    fd: &SFuncDef,
    gnv: &Tnv,
    mut ltnv: HashMap<String, OldType>,
) -> Result<OldType, io::Error> {
    //      Γ [e1 <- T1], ... [en <- Tn] ⊢ B : T2
    // -------------------------------------------------------
    //    Γ ⊢ (lambda e1:T1 ... en:Tn B) : (T1 * ... * Tn -> T2)

    fd.fps.iter().for_each(|(a, t)| {
        ltnv.insert(a.clone(), t.clone()); // Γ [e1 <- T1], ... [en <- Tn]
    });

    fd.body
        .iter()
        .map(|stmt| type_stmt(stmt, gnv, &mut ltnv))
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .try_fold(OldType::Void, |prev_t, next_t| match (&prev_t, next_t) {
            (_, OldType::Void) => Ok(prev_t),
            (OldType::Void, _) => Ok(next_t.clone()),
            (prev_t, next_t) => {
                if prev_t == next_t {
                    Ok(prev_t.clone())
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "type error"))
                }
            }
        })
        .and_then(|bt| {
            // ⊢ B : T2
            if bt == fd.typ {
                Ok(bt) // Γ ⊢ (lambda e1:T1 ... en:Tn B) : (T1 * ... * Tn -> T2)
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "type error"))
            }
        })
}

pub fn type_stmt(
    stmt: &SStmt,
    gnv: &Tnv,
    ltnv: &mut HashMap<String, OldType>,
) -> Result<OldType, io::Error> {
    match stmt {
        SStmt::IfEls { cond, then, els } => {
            let ct = type_expr(cond, gnv, ltnv)?;
            let tt = type_stmt(then, gnv, ltnv)?;
            let et = els.as_ref().map(|els| type_stmt(els, gnv, ltnv)).transpose()?;

            // todo (for now):
            // 3. tt and et are Stmt, not Vec<Stmt>
            // 4. must return expression Type. not void/stmt/valid/cmd Type

            match et {
                Some(et) => {
                    if ct == OldType::Bool && tt == et {
                        Ok(tt)
                    } else {
                        Err(io::Error::new(io::ErrorKind::Other, "type error"))
                    }
                }
                None => Ok(tt.clone()),
            }
        }
        SStmt::While { cond: _, body: _ } => todo!(),
        SStmt::Asnmt(vd) => {
            let et = type_expr(&vd.expr, gnv, ltnv)?;
            ltnv.insert(vd.alias.clone(), et.clone()); // Γ [x <- T]
            Ok(et)
        }
        SStmt::Return(expr) => {
            let rt = type_expr(expr, gnv, ltnv)?;
            Ok(rt)
        }
    }
}

pub fn type_expr(
    e: &SExpr,
    gtnv: &Tnv,
    ltnv: &HashMap<String, OldType>,
) -> Result<OldType, io::Error> {
    match e {
        // ---------------------intros (axioms)-------------------------
        SExpr::Int(_) => Ok(OldType::Int),   // ⊢ n : Int
        SExpr::Bool(_) => Ok(OldType::Bool), // ⊢ b : Bool
        // ---------------------elims (rules)--------------------------
        SExpr::UnaryE { op: _, l } => type_expr(l, gtnv, ltnv),
        SExpr::BinE { op, l, r } => match op {
            // ignoring distinctions within types
            SBinOp::Add | SBinOp::Sub | SBinOp::Mult | SBinOp::Div | SBinOp::Mod => {
                // ⊢ e1 : Int, ⊢ e2 : Int
                // ------------------------ BIN_OP
                //     ⊢ e1 + e2 : Int
                match (type_expr(l, gtnv, ltnv)?, type_expr(r, gtnv, ltnv)?) {
                    (OldType::Int, OldType::Int) => Ok(OldType::Int),
                    _ => Err(io::Error::new(io::ErrorKind::Other, "type error")),
                }
            } // perserves distinctions between types
        },
        SExpr::VarApp(alias) => ltnv // Γ ⊢ x: Γ(x)
            .get(alias)
            .cloned()
            .ok_or(io::Error::new(io::ErrorKind::Other, "type error")),
        SExpr::FuncApp { alias, aps: ap } => {
            //    Γ ⊢ f : (T1-> T2)      Γ ⊢ e : T1, ... Γ ⊢ e : Tn
            // ------------------------------------------------------- FUNC_APP
            //             Γ ⊢ f(e1, ... en) : T2

            // Γ ⊢ f : (T1-> T2)
            let f = gtnv
                .fnv
                .get(alias)
                .cloned()
                .ok_or(io::Error::new(io::ErrorKind::Other, "type error"))?;

            f.fp.iter()
                .zip(ap.iter())
                .map(|(fpt, ap)| {
                    type_expr(ap, gtnv, ltnv).and_then(|apt| {
                        //Γ ⊢ e : T1, ... Γ ⊢ e : Tn
                        if apt == *fpt {
                            Ok(())
                        } else {
                            Err(io::Error::new(io::ErrorKind::Other, "type error"))
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|_| f.body) // Γ ⊢ f(e) : T2
        }
        _ => Err(io::Error::new(io::ErrorKind::Other, "type error")),
    }
}

#[cfg(test)]
mod test_arith {
    use crate::lexer;
    use crate::parser_ast;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/statics-c0/arith";

    #[test]
    fn lit() {
        let chars = fs::read(format!("{TEST_DIR}/lit.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree).unwrap();
        insta::assert_yaml_snapshot!(typ, @r###"
        ---
        Int
        "###);
    }
}

#[cfg(test)]
mod test_control {
    use crate::lexer;
    use crate::parser_ast;
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
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree).unwrap();
        insta::assert_yaml_snapshot!(typ, @r###"
        ---
        Int
        "###);
    }

    #[test]
    fn ifels_wrong() {
        let chars = fs::read(format!("{TEST_DIR}/if2.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree);
        assert!(typ.is_err())
    }

    #[test]
    fn ifels_multi_side_effect() {
        let chars = fs::read(format!("{TEST_DIR}/if4.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree).unwrap();
        insta::assert_yaml_snapshot!(typ, @r###"
        ---
        Int
        "###);
    }

    #[test]
    fn ifels_multi_side_effect_wrong() {
        let chars = fs::read(format!("{TEST_DIR}/if5.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree);
        assert!(typ.is_err())
    }
}

#[cfg(test)]
mod test_bindings {
    use crate::lexer;
    use crate::parser_ast;
    use std::fs;

    const TEST_DIR: &str = "tests/fixtures/snap/statics-c0/bindings";

    #[test]
    fn func() {
        let chars = fs::read(format!("{TEST_DIR}/func.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree).unwrap();
        insta::assert_yaml_snapshot!(typ, @r###"
        ---
        Int
        "###);
    }

    #[test]
    fn func2() {
        let chars = fs::read(format!("{TEST_DIR}/func2.c0"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree);
        assert!(typ.is_err())
    }

    #[test]
    fn asnmt_expr() {
        const TEST_DIR: &str = "tests/fixtures/snap/shared/bindings";
        let chars = fs::read(format!("{TEST_DIR}/asnmt_expr.c"))
            .expect("file dne")
            .iter()
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let tokens = lexer::lex(&chars).unwrap();
        let tree = parser_ast::parse_prg(&tokens).unwrap();
        let typ = super::type_prg(&tree).unwrap();
        insta::assert_yaml_snapshot!(typ, @r###"
        ---
        Int
        "###);
    }
}
