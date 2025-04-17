use std::rc::Rc;
use super::{fresh_id, Instr, InstrKind, TypeKind};

#[derive(Clone, Debug)]
pub struct Int { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Int { pub fn new(ctrl: Box<dyn Instr>, typ: TypeKind) -> Self { Self { id: fresh_id(), typ, inputs: vec![Rc::new(ctrl)], outputs: vec![] } } }
impl Instr for Int { fn kind(&self) -> InstrKind { InstrKind::Int } }

#[derive(Debug, Clone)]
pub struct Add { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Add {
    pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(x), Rc::new(y)];
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![] }
    }

    fn x() -> Box<dyn Instr> { todo!() }
    fn y() -> Box<dyn Instr> { todo!() }
}
impl Instr for Add { fn kind(&self) -> InstrKind { InstrKind::Add } }

#[derive(Debug, Clone)]
pub struct Sub { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Sub {
    pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(x), Rc::new(y)];
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![] }
    }
    fn x() -> Box<dyn Instr> { todo!() }
    fn y() -> Box<dyn Instr> { todo!() }
}
impl Instr for Sub { fn kind(&self) -> InstrKind { InstrKind::Sub } }

#[derive(Debug, Clone)]
pub struct Mul { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Mul {
    pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(x), Rc::new(y)];
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![] }
    }
    fn x() -> Box<dyn Instr> { todo!() }
    fn y() -> Box<dyn Instr> { todo!() }
}
impl Instr for Mul { fn kind(&self) -> InstrKind { InstrKind::Mul } }

#[derive(Debug, Clone)]
pub struct Div { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
impl Div {
    pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
        let inputs = vec![Rc::new(x), Rc::new(y)];
        Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: vec![] }
    }
    fn x() -> Box<dyn Instr> { todo!() }
    fn y() -> Box<dyn Instr> { todo!() }
}
impl Instr for Div { fn kind(&self) -> InstrKind { InstrKind::Div } }

// #[rustfmt::skip]
// #[derive(Debug, Clone)]
// pub struct Neg { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<Box<dyn Instr>>>, pub outputs: Vec<Rc<Box<dyn Instr>>> }
// impl Neg {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Self {
//         let inputs = vec![Rc::new(x), Rc::new(y)];
//         let (x, y) = (inputs[0].clone(), inputs[1].clone());
//         Self { id: fresh_id(), typ: TypeKind::Bot, inputs, outputs: todo!() }
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