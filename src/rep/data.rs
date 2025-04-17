// ************************************ DATA ***********************************

use std::rc::Rc;
use super::{ctl::Start, fresh_id, Instr, InstrKind, TypeKind};

#[rustfmt::skip] 
#[derive(Clone, Debug)]
pub struct Int { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Int {
    pub fn new(ctrl: Box<dyn Instr>, typ: TypeKind) -> Self {
        Self {
            id: fresh_id(),
            typ,
            inputs: vec![Rc::new(ctrl)], // phantom edge to start enabling graph traversal
            outputs: vec![],
        }
    }
}
impl Instr for Int { fn kind(&self) -> InstrKind { InstrKind::Int } }


#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Add { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, x: Rc<Box<dyn Instr>>, y: Rc<Box<dyn Instr>> }
impl Add {
    pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(x), Rc::new(y)];
        let (x, y) = (inputs[0].clone(), inputs[1].clone());
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![], x, y }
    }
}
impl Instr for Add { fn kind(&self) -> InstrKind { InstrKind::Add } }

// #[rustfmt::skip]
// #[derive(Debug, Clone)]
// pub struct Sub { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, x: Rc<Box<dyn Instr>>, y: Rc<Box<dyn Instr>> }
// impl Sub {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
//         let inputs = vec![Rc::new(x), Rc::new(y)];
//         let (x, y) = (inputs[0].clone(), inputs[1].clone());
//         Self { id: fresh_id(), typ: todo!(), inputs, outputs: todo!(), x, y }
//     }
// }
// impl Instr for Sub {
//     fn add_input(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }

//     fn add_output(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }
// }

// #[rustfmt::skip]
// #[derive(Debug, Clone)]
// pub struct Mul { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, x: Rc<Box<dyn Instr>>, y: Rc<Box<dyn Instr>> }
// impl Mul {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
//         let inputs = vec![Rc::new(x), Rc::new(y)];
//         let (x, y) = (inputs[0].clone(), inputs[1].clone());
//         Self { id: fresh_id(), typ: todo!(), inputs, outputs: todo!(), x, y }
//     }
// }
// impl Instr for Mul {
//     fn add_input(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }

//     fn add_output(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }
// }

// #[rustfmt::skip]
// #[derive(Debug, Clone)]
// pub struct Div { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, x: Rc<Box<dyn Instr>>, y: Rc<Box<dyn Instr>> }
// impl Div {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
//         let inputs = vec![Rc::new(x), Rc::new(y)];
//         let (x, y) = (inputs[0].clone(), inputs[1].clone());
//         Self { id: fresh_id(), typ: todo!(), inputs, outputs: todo!(), x, y }
//     }
// }
// impl Instr for Div {
//     fn add_input(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }

//     fn add_output(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }
// }

// #[rustfmt::skip]
// #[derive(Debug, Clone)]
// pub struct Neg { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>>, x: Rc<Box<dyn Instr>>, y: Rc<Box<dyn Instr>> }
// impl Neg {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
//         let inputs = vec![Rc::new(x), Rc::new(y)];
//         let (x, y) = (inputs[0].clone(), inputs[1].clone());
//         Self { id: fresh_id(), typ: todo!(), inputs, outputs: todo!(), x, y }
//     }
// }

// impl Instr for Neg {
//     fn add_input(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }

//     fn add_output(&mut self, input: Box<dyn Instr>) -> () {
//         todo!()
//     }
// }