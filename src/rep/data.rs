use std::{cell::RefCell, rc::{Rc, Weak}};
use super::{fresh_id, Instr, InstrKind, TypeKind};

#[derive(Clone, Debug)]
pub struct Int { _id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Int {
    pub fn new(ctrl: Rc<dyn Instr>, typ: TypeKind) -> Rc<Self> {
        let instr = Rc::new(Self{ _id: fresh_id(), typ, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![ctrl]);
        instr
    }
}
impl Instr for Int {
    fn kind(&self) -> InstrKind { InstrKind::Int }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn eval_type(&self) -> TypeKind { self.typ }
}

#[derive(Debug, Clone)]
pub struct Add { _id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Add {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }

    fn x(&self) -> Rc<dyn Instr> { self.inputs.borrow()[0].clone() }
    fn y(&self) -> Rc<dyn Instr> { self.inputs.borrow()[1].clone() }
}
impl Instr for Add {
    fn kind(&self) -> InstrKind { InstrKind::Add }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    
    fn eval_type(&self) -> TypeKind {
        println!("moose, {:?}, {:?}", self.x().eval_type(), self.y().eval_type());
        match (self.x().eval_type(), self.y().eval_type()) {
            (TypeKind::Int(x), TypeKind::Int(y)) => TypeKind::Int(x + y),
            _ => TypeKind::Bot,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Sub { _id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Sub {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Sub {
    fn kind(&self) -> InstrKind { InstrKind::Sub }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
}


#[derive(Debug, Clone)]
pub struct Mul { _id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Mul {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Mul {
    fn kind(&self) -> InstrKind { InstrKind::Mul }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
}


#[derive(Debug, Clone)]
pub struct Div { _id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Div {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeKind::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Div {
    fn kind(&self) -> InstrKind { InstrKind::Div }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
}


// #[derive(Debug, Clone)]
// pub struct Neg { id: i128, typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
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