pub mod ctl;
pub mod data;
pub mod scope;

use std::{any::Any, cell::RefCell, fmt::Debug, rc::{Rc, Weak}};
use data::Int;

// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 { unsafe { ID += 1; ID } }

/// NB1: access pattern of the graph require mutability after node construction
///      for peephole optimizations — the workhorse of sea of nodes. as a result,
///      all nodes are referenced with rcs and have interior mutability with
///      refcell, as opposed to a two-phase approach with &mut methods during a
///      factory phase and rc methods once the graph has been constructed.
/// 
///      although smelly from rust's ownership perspective, rc/arc<refcell<_>>
///      everywhere is inevitable (todo is it?) with mutable graphs in rust since
///      ownership and linear types effectively places a DAG (tree) on the static
///      type system.
///      TODO: alternatives to rc<refcell<_>>
/// NB2: all instruction constructors that implement the instr trait must call
///      .add_children() — this invariant is enforced by the human reviewer??
///      type state combined with field encapsulation can be used to enforce
///      callers outside the module, but does not enforce within the module for
///      library implementors
/// NB3: since the generics with trait bounds get monomorphized (static polymorphism),
///      trait objects (dynamic polymorphism) are used because sea of nodes
///      heterogeneity requires dynamic dispatch. however, since trait objects *only*
///      provide polymorphism on behavior and not data, the required trait's methods
///      "lift" individual struct data (which have interior mutability)

///      an alternative design is to represent the node as a single instr enum with
///      variant-specific data would need to do the opposite,
///      and "lower" shared behavior:
enum Instr {
    Add(Box<Instr>, Box<Instr>), Sub, // arithmetic
    VarDef, VarUse(String), // bindings
    Start, Stop, Branch, Loop, // ctrl
}

fn peephole(i: Box<Instr>) -> Box<Instr> {
    match *i {
        Instr::Add(instr, instr1) => todo!(),
        Instr::Sub => todo!(),
        Instr::VarDef => todo!(),
        Instr::VarUse(_) => todo!(),
        Instr::Start => todo!(),
        Instr::Stop => todo!(),
        Instr::Branch => todo!(),
        Instr::Loop => todo!(),
    }
    todo!()
}

// a heterogenous graph implemented in rust is nasty regardless of how you
// slice&dice, given that you're fighting against linear types.

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAndVal { Bot, Top, Simple, Int(i128), Tup(Vec<Box<Self>>) } // see: https://en.wikipedia.org/wiki/Lattice_(order)
impl TypeAndVal {
    pub fn is_constant(&self) -> bool {
        match self {
            TypeAndVal::Bot => false,
            TypeAndVal::Top => true,
            TypeAndVal::Simple => todo!(),
            TypeAndVal::Int(_) => true,
            TypeAndVal::Tup(type_and_vals) => todo!(),
        }
    }
}

pub enum InstrKind { Start, Return, Int, Add, Sub, Mul, Div, Scope }
pub trait InstrNode : Debug + Any {
    fn kind(&self) -> InstrKind;
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>>;
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>>;
    fn eval_type(&self) -> TypeAndVal { TypeAndVal::Bot }
    fn idealize(&self) -> Rc<dyn InstrNode>;

    fn push_weak_self_on_children(self: &Rc<Self>) where Self: Sized + 'static {
        for i in self.uses().borrow().iter() {
            i.used().borrow_mut().push(Rc::downgrade(&(self.clone() as Rc<dyn InstrNode>)));
        }
    }

    fn add_children(self: &Rc<Self>, children: Vec<Rc<dyn InstrNode>>) where Self: Sized + 'static  {
        for c in children {
            self.add_child(c);
        }
    }

    fn add_child(self: &Rc<Self>, child: Rc<dyn InstrNode>) where Self: Sized + 'static {
        self.uses().borrow_mut().push(child);
        let child = &self.uses().borrow()[self.uses().borrow().len()-1];
        child.used().borrow_mut().push(Rc::downgrade(&(self.clone() as Rc<dyn InstrNode>)));
    }

    fn peephole(self: Rc<Self>, start: Rc<dyn InstrNode>) -> Rc<dyn InstrNode> where Self: Sized + 'static {
        let typ = self.eval_type();
        let instr: Rc<dyn InstrNode> = match self.kind() {
            InstrKind::Int => self,
            _ => if typ.is_constant() { Int::new(start.clone(), typ) } else { self },
        };
        let instr = instr.idealize();

        return instr;
    } // NB1: self is Drop if instr no longer aliases it, possibly cascading to children
      // NB2: edge maintenance with weak pointers on on the children of the
      //      dropped instr are left for now which can be cleared eagerly
      //      (from parent's dealloc) or lazily (on child's processing) in the future
}

// NB: multiinstr are instructions that produce a tuple-valued result, and
//     projections are nodes which act as named edges to extract values from
//     the multiinstr which lets us keep actual edges lightweight (rcs).
//     projections have no corresponding runtime operation. they "execute" in
//     zero cycles.
pub trait MultiInstr : InstrNode {}
#[derive(Debug, Clone)]
pub struct Proj { _id: i128, inputs: RefCell<Vec<Rc<dyn InstrNode>>>, pub outputs: RefCell<Vec<Weak<dyn InstrNode>>>, i: usize}
impl Proj {
    pub fn new(multi: Rc<dyn InstrNode>, i: usize) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]), i });
        instr.add_children(vec![multi]);
        instr
    }
    fn _x() -> Rc<dyn InstrNode> { todo!() }
    fn _y() -> Rc<dyn InstrNode> { todo!() }
}
impl InstrNode for Proj {
    fn kind(&self) -> InstrKind { InstrKind::Div }
    fn uses(&self) -> &RefCell<Vec<Rc<dyn InstrNode>>> { &self.inputs }
    fn used(&self) -> &RefCell<Vec<Weak<dyn InstrNode>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn InstrNode> { todo!() }
}

// #[derive(Debug, Clone)]
// pub struct MultiInstr { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>> }
// impl MultiInstr {
//     pub fn new() -> Rc<Self> {
//         let instr = Rc::new(Self{ _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]) });
//         instr
//     }
// }
// impl Instr for MultiInstr {
//     fn kind(&self) -> InstrKind { InstrKind::MultiInstr }
//     fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
//     fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
//     fn idealize(&self) -> Rc<dyn Instr> { todo!() }
// }