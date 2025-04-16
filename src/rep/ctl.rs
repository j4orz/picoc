// ********************************** CONTROL **********************************
// NB: control instructions still have a type because the sea of nodes
// representation is homogenous (instruction agnostic). control instructions
// can be peephole optimized with TODO: (phi functions.)

use std::sync::Arc;
use super::{fresh_id, Instr, Type};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct StartFields {
    pub id: i128, pub typ: Type,
}
impl StartFields {
    fn new() -> Self {
        Self { id: fresh_id(), typ: Type::Bot }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct ReturnFields { pub id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, ctrl: Arc<Instr>, data: Arc<Instr> }
impl ReturnFields {
    pub fn new(ctrl: Arc<Instr>, data: Instr) -> Self {
        let ud = vec![ctrl, Arc::new(data)];
        let (ctrl, data) = (ud[0].clone(), ud[1].clone());

        Self { id: fresh_id(), typ: Type::Bot, ud, du: vec![], ctrl, data }
    }
}