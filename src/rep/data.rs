use std::{cell::RefCell, rc::{Rc, Weak}};
use super::{fresh_id, InstrNode, InstrKind, MultiInstr, TypeAndVal};

// NB: constants set start instruction as input to enable forward graph walk. TODO
//     the edge carries no semantic meaning.
#[derive(Clone, Debug)]
pub struct Int { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Int {
    pub fn new(ctrl: Rc<dyn InstrNode>, typ: TypeAndVal) -> Rc<Self> {
        let instr = Rc::new(Self{ _id: fresh_id(), typ, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![ctrl]);
        instr
    }
}
impl InstrNode for Int {
    fn kind(&self) -> InstrKind { InstrKind::Int }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    // fn eval_type(&self) -> TypeAndVal { self.typ } TODO: fixme for constant propagation
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Add { _id: i128, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Add {
    pub fn new(x: Rc<dyn InstrNode>, y: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
}
impl InstrNode for Add {
    fn kind(&self) -> InstrKind { InstrKind::Add }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    
    fn eval_type(&self) -> TypeAndVal {
        let (x,y) = (self.inputs.borrow()[0].clone(), self.inputs.borrow()[1].clone());
        match (x.eval_type(), y.eval_type()) {
            (TypeAndVal::Int(x), TypeAndVal::Int(y)) => TypeAndVal::Int(x + y),
            _ => TypeAndVal::Bot,
        }
    }
    
    fn idealize(&self) -> Rc<dyn InstrNode> {
        let (x,y) = (self.inputs.borrow()[0].clone(), self.inputs.borrow()[1].clone());
        let (x_type, y_type) = (x.eval_type(), y.eval_type());
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Sub { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Sub {
    pub fn new(x: Rc<dyn InstrNode>, y: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn InstrNode> { todo!() }
    fn _y() -> Rc<dyn InstrNode> { todo!() }
}
impl InstrNode for Sub {
    fn kind(&self) -> InstrKind { InstrKind::Sub }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Mul { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Mul {
    pub fn new(x: Rc<dyn InstrNode>, y: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn InstrNode> { todo!() }
    fn _y() -> Rc<dyn InstrNode> { todo!() }
}
impl InstrNode for Mul {
    fn kind(&self) -> InstrKind { InstrKind::Mul }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}


#[derive(Debug, Clone)]
pub struct Div { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Div {
    pub fn new(x: Rc<dyn InstrNode>, y: Rc<dyn InstrNode>) -> Rc<dyn InstrNode> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn InstrNode> { todo!() }
    fn _y() -> Rc<dyn InstrNode> { todo!() }
}
impl InstrNode for Div {
    fn kind(&self) -> InstrKind { InstrKind::Div }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Phi { _id: i128, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Phi {
    pub fn new(ctrl: Rc<dyn InstrNode>) -> Rc<dyn InstrNode> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![ctrl]);
        instr
    }
    fn _x() -> Rc<dyn InstrNode> { todo!() }
    fn _y() -> Rc<dyn InstrNode> { todo!() }
}
impl InstrNode for Phi {
    fn kind(&self) -> InstrKind { InstrKind::Div }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

// #[derive(Debug, Clone)]
// pub struct Bool { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
// impl Bool {
//     pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
//         let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
//         instr.add_children(vec![x, y]);
//         instr
//     }
//     fn _x() -> Rc<dyn Instr> { todo!() }
//     fn _y() -> Rc<dyn Instr> { todo!() }
// }
// impl Instr for Bool {
//     fn kind(&self) -> InstrKind { InstrKind::Div }
//     fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
//     fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
//     fn idealize(&self) -> Rc<dyn Instr> { todo!() }
// }

// #[derive(Debug, Clone)]
// pub struct Not { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
// impl Not {
//     pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
//         let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
//         instr.add_children(vec![x, y]);
//         instr
//     }
//     fn _x() -> Rc<dyn Instr> { todo!() }
//     fn _y() -> Rc<dyn Instr> { todo!() }
// }
// impl Instr for Not {
//     fn kind(&self) -> InstrKind { InstrKind::Div }
//     fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
//     fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
//     fn idealize(&self) -> Rc<dyn Instr> { todo!() }
// }

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