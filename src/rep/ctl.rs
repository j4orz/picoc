// NB1: control instructions still have a type because the sea of nodes
//      representation is homogenous (instruction agnostic). control instructions
//      can be peephole optimized with TODO: (phi functions.)

// NB2: these instructions make up a control subgraph (petri net) where a single
//      control pointer moves from instruction to instruction as execution
//      proceeds. this pointer is stored in the scope's first (outermost) nv.
//      the start instruction replaces the start basic block, and the control
//      pointer can only proceed to one of {branch, region, stop}. // TODO: enforce this invariant?

use std::{cell::RefCell, rc::{Rc, Weak}};
use super::{fresh_id, Instr, OpCode, MultiInstr, TypeAndVal};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct Start { id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Start {
    pub fn new(types: Vec<Box<TypeAndVal>>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Tup(types), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
        instr
    }
}
impl Instr for Start {
    fn kind(&self) -> OpCode { OpCode::Start }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}
impl MultiInstr for Start {
    
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Branch { id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Branch {
    pub fn new(ctrl: Rc<dyn Instr>, pred: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, pred]);
        instr
    }

    fn _ctl() -> Box<dyn Instr> { todo!() }
    fn _data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Branch {
    fn kind(&self) -> OpCode { todo!() }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Region { id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Region {
    pub fn new(ctrl: Rc<dyn Instr>, pred: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, pred]);
        instr
    }

    fn _ctl() -> Box<dyn Instr> { todo!() }
    fn _data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Region {
    fn kind(&self) -> OpCode { todo!() }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Return { id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Return {
    pub fn new(ctrl: Rc<dyn Instr>, data: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, data]);
        instr
    }

    fn _ctl() -> Box<dyn Instr> { todo!() }
    fn _data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Return {
    fn kind(&self) -> OpCode { OpCode::Return }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Stop { id: i128, typ: TypeAndVal, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
impl Stop {
    pub fn new(ctrl: Rc<dyn Instr>, data: Rc<dyn Instr>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, data]);
        instr
    }

    fn _ctl() -> Box<dyn Instr> { todo!() }
    fn _data() -> Box<dyn Instr> { todo!() }
}
impl Instr for Stop {
    fn kind(&self) -> OpCode { todo!() }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
}