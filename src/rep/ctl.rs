// NB: control instructions still have a type because the sea of nodes
// representation is homogenous (instruction agnostic). control instructions
// can be peephole optimized with TODO: (phi functions.)

use std::{rc::Rc, sync::Arc};
use super::{fresh_id, Instr, InstrKind, TypeKind};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct Start { pub id: i128, pub typ: TypeKind, pub inputs: Vec<Arc<dyn Instr>>, pub outputs: Vec<Arc<dyn Instr>> }
impl Start { pub fn new() -> Self { Self { id: fresh_id(), typ: TypeKind::Bot, inputs: vec![], outputs: vec![] }}}
impl Instr for Start { fn kind(&self) -> InstrKind { InstrKind::Start } }

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Return { pub id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Return {
    pub fn new(ctrl: Rc<Box<dyn Instr>>, data: Box<dyn Instr>) -> Self {
        let inputs = vec![ctrl, Rc::new(data)];
        let (_ctrl, _data) = (inputs[0].clone(), inputs[1].clone());

        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![]}
    }

    fn ctl() -> Box<dyn Instr> { todo!() }
    fn data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Return { fn kind(&self) -> InstrKind { InstrKind::Return } }