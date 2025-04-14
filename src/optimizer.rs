use crate::{AddFields, Instr, Type};

impl Instr {
    pub fn peephole(&self) -> Self {
        let typ = self.eval_type();
        // optimizations
        // - dead code elimination (DCE)
        // - common subexpression elimination (CSE)
        // - constant folding, constant propagation
        // - global value numbering
        todo!()
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
