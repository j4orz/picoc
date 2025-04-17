use std::{cell::RefCell, rc::Rc};
use super::{fresh_id, Instr, InstrKind, TypeKind};

#[derive(Clone, Debug)]
pub struct Int { _id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Int {
    pub fn new(ctl: Rc<dyn Instr>, typ: TypeKind) -> Rc<dyn Instr> {
        let instr = Rc::new(Self{ _id: fresh_id(), typ, inputs: vec![ctl], outputs: RefCell::new(vec![]) });
        instr.init_outputs();
        instr
    }
}
impl Instr for Int {
    fn kind(&self) -> InstrKind { InstrKind::Int }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}

#[derive(Debug, Clone)]
pub struct Add { _id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Add {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: vec![x, y], outputs: RefCell::new(vec![]) });
        instr.init_outputs();
        instr
    }

    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Add {
    fn kind(&self) -> InstrKind { InstrKind::Add }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}


#[derive(Debug, Clone)]
pub struct Sub { _id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Sub {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: vec![x, y], outputs: RefCell::new(vec![]) });
        instr.init_outputs();
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Sub {
    fn kind(&self) -> InstrKind { InstrKind::Sub }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}


#[derive(Debug, Clone)]
pub struct Mul { _id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Mul {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: vec![x, y], outputs: RefCell::new(vec![]) });
        instr.init_outputs();
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Mul {
    fn kind(&self) -> InstrKind { InstrKind::Mul }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}


#[derive(Debug, Clone)]
pub struct Div { _id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
impl Div {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: vec![x, y], outputs: RefCell::new(vec![]) });
        instr.init_outputs();
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Div {
    fn kind(&self) -> InstrKind { InstrKind::Div }
    fn inputs(&self) -> &Vec<Rc<dyn Instr>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.outputs }
}


// #[derive(Debug, Clone)]
// pub struct Neg { id: i128, pub typ: TypeKind, pub inputs: Vec<Rc<dyn Instr>>, pub outputs: RefCell<Vec<Rc<dyn Instr>>> }
// impl Neg {
//     pub fn new(x: Box<dyn Instr>, y: Box<dyn Instr>) -> Rc<dyn Instr> {
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