use std::{cell::RefCell, rc::{Rc, Weak}};
use super::{fresh_id, Instr, OpCode, MultiInstr, TypeAndVal};

// NB: constants set start instruction as input to enable forward graph walk. TODO
//     the edge carries no semantic meaning.
#[derive(Clone, Debug)]
pub struct Int { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Int {
    pub fn new(ctrl: Rc<dyn Instr>, typ: TypeAndVal) -> Rc<Self> {
        let instr = Rc::new(Self{ _id: fresh_id(), typ, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![ctrl]);
        instr
    }
}
impl Instr for Int {
    fn kind(&self) -> OpCode { OpCode::Int }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    // fn eval_type(&self) -> TypeAndVal { self.typ } TODO: fixme for constant propagation
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Add { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Add {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
}
impl Instr for Add {
    fn kind(&self) -> OpCode { OpCode::Add }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    
    fn eval_type(&self) -> TypeAndVal {
        let (x,y) = (self.inputs.borrow()[0].clone(), self.inputs.borrow()[1].clone());
        match (x.eval_type(), y.eval_type()) {
            (TypeAndVal::Int(x), TypeAndVal::Int(y)) => TypeAndVal::Int(x + y),
            _ => TypeAndVal::Bot,
        }
    }
    
    fn idealize(&self) -> Rc<dyn Instr> {
        let (x,y) = (self.inputs.borrow()[0].clone(), self.inputs.borrow()[1].clone());
        let (x_type, y_type) = (x.eval_type(), y.eval_type());
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct Sub { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Sub {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Sub {
    fn kind(&self) -> OpCode { OpCode::Sub }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Mul { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Mul {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Mul {
    fn kind(&self) -> OpCode { OpCode::Mul }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}


#[derive(Debug, Clone)]
pub struct Div { _id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Div {
    pub fn new(x: Rc<dyn Instr>, y: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![x, y]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Div {
    fn kind(&self) -> OpCode { OpCode::Div }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[derive(Debug, Clone)]
pub struct Phi { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Phi {
    pub fn new(ctrl: Rc<dyn Instr>) -> Rc<dyn Instr> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr.add_children(vec![ctrl]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Phi {
    fn kind(&self) -> OpCode { OpCode::Div }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
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