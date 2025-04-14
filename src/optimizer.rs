use std::sync::Arc;

use crate::{parser::START, AddFields, ConstantFields, Instr, Type};

impl Instr {
    // many optimizations become peepholes
    // with sea of nodes representation:
    // 1. dead code elimination (DCE)
    // 2. common subexpression elimination (CSE)
    pub fn peephole(self) -> Self {
        let typ = self.eval_type();

        // -constant folding, constant propagation
        let instr = match self {
            Instr::Constant(_) => self,
            _ => if typ == Type::Top { Instr::Constant(ConstantFields::new(Arc::new(START.clone()), typ)) } else { self },
        };

        // -global value numbering

        return instr;
    }

    fn eval_type(&self) -> Type {
        // typer.rs
        match self {
            Instr::Start(start_fields) => todo!(),
            Instr::Return(return_fields) => todo!(),
            Instr::Constant(constant_fields) => constant_fields.typ,
            Instr::Add(AddFields { x, y, .. }) => match (x.eval_type(), y.eval_type()) {
                (Type::Int(x), Type::Int(y)) => Type::Int(x + y),
                _ => Type::Bot,
            },
            Instr::Sub(sub_fields) => todo!(),
            Instr::Mul(mul_fields) => todo!(),
            Instr::Div(div_fields) => todo!(),
            Instr::Neg(neg_fields) => todo!(),
        }
    }
    fn idealize(&self) {}
}
