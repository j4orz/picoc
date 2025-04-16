use std::sync::Arc;
use crate::{parser::START, rep::{Type, Instr}};

pub fn peephole(instr: Box<dyn Instr>) -> Box<dyn Instr> { // many optimizations become peepholes with sea of nodes representation:
    // let typ = self.eval_type();
    // let instr = match self { // -constant propagation/folding (modest dead code elimination)
    //     Instr::Constant(_) => self,
    //     _ => if typ.is_constant() { Instr::Constant(ConstantFields::new(Arc::new(START.clone()), typ)) } else { self },
    // };
    // return instr;
    todo!()
}

pub fn eval_type(instr: Box<dyn Instr>) -> Box<dyn Instr> {
    todo!()
}



    // fn eval_type(&self) -> Type {
    //     // typer.rs
    //     match self {
    //         Instr::Scope(scope_fields) => todo!(),
    //         Instr::Start(start_fields) => todo!(),
    //         Instr::Return(return_fields) => todo!(),
    //         Instr::Constant(constant_fields) => constant_fields.typ,
    //         Instr::Add(AddFields { x, y, .. }) => match (x.eval_type(), y.eval_type()) {
    //             // ⊢ e1 : Int, ⊢ e2 : Int
    //             // ------------------------ BIN_OP
    //             //     ⊢ e1 + e2 : Int
    //             (Type::Int(x), Type::Int(y)) => Type::Int(x + y),
    //             _ => Type::Bot,
    //         },
    //         Instr::Sub(sub_fields) => todo!(),
    //         Instr::Mul(mul_fields) => todo!(),
    //         Instr::Div(div_fields) => todo!(),
    //         Instr::Neg(neg_fields) => todo!(),
    //     }
    // }
    // fn idealize(&self) {}
