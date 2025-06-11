// NB1: control instruction nodes still have a type because the SoN IR is homogenous
//      this means control instructions can be peephole optimized with TODO: (phi functions.)??
// NB2: control instruction nodes make up a control subgraph (petri net) where a
//      single control pointer moves from control node to control node as
//      execution proceeds. (like rock climbing)

//      this pointer is stored in the scope's first (outermost) nv. the start
//      instruction replaces the start basic block, and the control pointer can
//      only proceed to one of {branch, region, stop}. // TODO: enforce this invariant?

use std::{cell::RefCell, rc::{Rc, Weak}};
use super::{fresh_id, InstrNode, InstrKind, MultiInstr, TypeAndVal};

#[derive(Debug, Clone)]
#[rustfmt::skip] pub struct Start {id: i128, typ: TypeAndVal, uses: RefCell<Vec<Rc<dyn InstrNode>>>, pub used: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Start {
    pub fn new(_types: Vec<Box<TypeAndVal>>) -> Rc<Self> {
        //  TypeAndVal::Tup(types)
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, uses: RefCell::new(vec![]), used: RefCell::new(vec![]) });
        instr
    }
}
impl InstrNode for Start {
    fn kind(&self) -> InstrKind { InstrKind::Start }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.uses }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.used }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}
impl MultiInstr for Start {
    
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Branch { id: i128, typ: TypeAndVal, uses: RefCell<Vec<Rc<dyn InstrNode>>>, pub used: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Branch {
    pub fn new(ctrl: Rc<dyn InstrNode>, pred: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, uses: RefCell::new(vec![]), used: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, pred]);
        instr
    }

    fn _ctl() -> Box<dyn InstrNode> { todo!() }
    fn _data() -> Box<dyn InstrNode> { todo!() }
}
impl InstrNode for Branch {
    fn kind(&self) -> InstrKind { todo!() }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.uses }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.used }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Region { id: i128, typ: TypeAndVal, uses: RefCell<Vec<Rc<dyn InstrNode>>>, pub used: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Region {
    pub fn new(ctrl: Rc<dyn InstrNode>, pred: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, uses: RefCell::new(vec![]), used: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, pred]);
        instr
    }

    fn _ctl() -> Box<dyn InstrNode> { todo!() }
    fn _data() -> Box<dyn InstrNode> { todo!() }
}
impl InstrNode for Region {
    fn kind(&self) -> InstrKind { todo!() }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.uses }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.used }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Return { id: i128, typ: TypeAndVal, uses: RefCell<Vec<Rc<dyn InstrNode>>>, pub used: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Return {
    pub fn new(ctrl: Rc<dyn InstrNode>, data: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, uses: RefCell::new(vec![]), used: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, data]);
        instr
    }

    fn _ctl() -> Box<dyn InstrNode> { todo!() }
    fn _data() -> Box<dyn InstrNode> { todo!() }
}
impl InstrNode for Return {
    fn kind(&self) -> InstrKind { InstrKind::Return }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.uses }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.used }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct Stop { id: i128, typ: TypeAndVal, uses: RefCell<Vec<Rc<dyn InstrNode>>>, pub used: RefCell<Vec<Weak<dyn InstrNode>>> }
impl Stop {
    pub fn new(ctrl: Rc<dyn InstrNode>, data: Rc<dyn InstrNode>) -> Rc<Self> {
        let instr = Rc::new(Self { id: fresh_id(), typ: TypeAndVal::Bot, uses: RefCell::new(vec![]), used: RefCell::new(vec![])});
        instr.add_children(vec![ctrl, data]);
        instr
    }

    fn _ctl() -> Box<dyn InstrNode> { todo!() }
    fn _data() -> Box<dyn InstrNode> { todo!() }
}
impl InstrNode for Stop {
    fn kind(&self) -> InstrKind { todo!() }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.uses }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.used }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}