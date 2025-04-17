// NB: control instructions still have a type because the sea of nodes
// representation is homogenous (instruction agnostic). control instructions
// can be peephole optimized with TODO: (phi functions.)

use std::{cell::RefCell, rc::Rc};
use super::{fresh_id, Instr, InstrKind, TypeKind};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct Start { pub id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Start { pub fn new() -> Self { Self { id: fresh_id(), typ: TypeKind::Bot, inputs: vec![], outputs: RefCell::new(vec![]) }}}
impl Instr for Start {
    fn kind(&self) -> InstrKind { InstrKind::Start }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}
#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Return { pub id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Return {
    pub fn new(ctrl: Rc<dyn Instr>, data: Rc<dyn Instr>) -> Self {
        let inputs = vec![ctrl, data];
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: RefCell::new(vec![])}
    }

    fn _ctl() -> Box<dyn Instr> { todo!() }
    fn _data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Return {
    fn kind(&self) -> InstrKind { InstrKind::Return }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}