// ********************************** CONTROL **********************************
// NB: control instructions still have a type because the sea of nodes
// representation is homogenous (instruction agnostic). control instructions
// can be peephole optimized with TODO: (phi functions.)

use std::rc::Rc;
use super::{fresh_id, Instr, Type};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct Start { pub id: i128, pub typ: Type, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: Vec<Rc<dyn Instr>> }
impl Start {
    fn new() -> Self {
        Self { id: fresh_id(), typ: Type::Bot, inputs: vec![], outputs: vec![] }
    }
}

impl Instr for Start {
    fn add_input(&mut self, input: Box<dyn Instr>) -> () {
        todo!()
    }

    fn add_output(&mut self, input: Box<dyn Instr>) -> () {
        todo!()
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Return { pub id: i128, pub typ: Type, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, ctrl: Rc<Box<dyn Instr>>, data: Rc<Box<dyn Instr>> }
impl Return {
    pub fn new(ctrl: Box<dyn Instr>, data: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(ctrl), Rc::new(data)];
        let (ctrl, data) = (inputs[0].clone(), inputs[1].clone());

        Self { id: fresh_id(), typ: Type::Bot, inputs, outputs: vec![], ctrl, data }
    }
}

impl Instr for Return {
    fn add_input(&mut self, input: Box<dyn Instr>) -> () {
        todo!()
    }

    fn add_output(&mut self, input: Box<dyn Instr>) -> () {
        todo!()
    }
}