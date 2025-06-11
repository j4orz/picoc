pub mod ctl;
pub mod data;
pub mod scope;

use std::{any::Any, cell::RefCell, fmt::Debug, rc::{Rc, Weak}};
use data::Int;

/// NB1: access pattern of the graph require mutability after node construction
///      for peephole optimizations — the workhorse of sea of nodes. as a result,
///      all nodes are referenced with rcs and have interior mutability with
///      refcell, as opposed to a two-phase approach with &mut methods during a
///      factory phase and rc methods once the graph has been constructed.
///      although smelly from rust's ownership perspective, rc<refcell<_>>
///      everywhere is inevitable with mutable graphs in rust since ownership
///      is effectively placing a DAG (tree) on the static type system. if the
///      compiler becomes multithreaded then rc's need to be converted to arcs.
///      TODO: alternatives to rc<refcell<_>>

/// NB2: all instruction constructors that implement the instr trait must call
///      .add_children() — this is invariant is enforced by the human reviewer.
///      type state combined with field encapsulation can be used to enforce
///      callers outside the module, but does not enforce within the module for
///      library implementors

// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 { unsafe { ID += 1; ID } }

// since the generics with trait bounds get monomorphized (static polymorphism),
// trait objects (dynamic polymorphism) is used because sea of nodes
// heterogeneity requires dynamic dispatch. since trait objects only provide
// polymorphism on behavior and not data, required methods lift individual struct
// data (which have interior mutability) to shared trait behavior. a single instr
// enum with variant-specific data would need to do the opposite, and "lower"
// shared behavior. a heterogenous graph implemented in rust is nasty regardless
// of how you slice/dice it given that you're effectively fighting against linear types.
// TODO: research more into alternatives into heterogenous graphs in rust. petagraph??

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAndVal { Bot, Top, Simple, Int(i128), Tup(Vec<Box<Self>>) } // see: https://en.wikipedia.org/wiki/Lattice_(order)
pub enum OpCode { Start, Return, Int, Add, Sub, Mul, Div, Scope }

pub trait Instr : Debug + Any {
    fn kind(&self) -> OpCode;
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>>;
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>>;
    fn eval_type(&self) -> TypeAndVal { TypeAndVal::Bot }
    fn idealize(&self) -> Rc<dyn Instr>;

    fn push_weak_self_on_children(self: &Rc<Self>) where Self: Sized + 'static {
        for i in self.inputs().borrow().iter() {
            i.outputs().borrow_mut().push(Rc::downgrade(&(self.clone() as Rc<dyn Instr>)));
        }
    }

    fn add_children(self: &Rc<Self>, children: Vec<Rc<dyn Instr>>) where Self: Sized + 'static  {
        for c in children {
            self.add_child(c);
        }
    }

    fn add_child(self: &Rc<Self>, child: Rc<dyn Instr>) where Self: Sized + 'static {
        self.inputs().borrow_mut().push(child);
        let child = &self.inputs().borrow()[self.inputs().borrow().len()-1];
        child.outputs().borrow_mut().push(Rc::downgrade(&(self.clone() as Rc<dyn Instr>)));
    }

    fn peephole(self: Rc<Self>, start: Rc<dyn Instr>) -> Rc<dyn Instr> where Self: Sized + 'static {
        let typ = self.eval_type();
        let instr: Rc<dyn Instr> = match self.kind() {
            OpCode::Int => self,
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
pub trait MultiInstr : Instr {}
#[derive(Debug, Clone)]
pub struct Proj { _id: i128, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>>, i: usize}
impl Proj {
    pub fn new(multi: Rc<dyn Instr>, i: usize) -> Rc<Self> {
        let instr = Rc::new(Self { _id: fresh_id(), inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]), i });
        instr.add_children(vec![multi]);
        instr
    }
    fn _x() -> Rc<dyn Instr> { todo!() }
    fn _y() -> Rc<dyn Instr> { todo!() }
}
impl Instr for Proj {
    fn kind(&self) -> OpCode { OpCode::Div }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
    fn idealize(&self) -> Rc<dyn Instr> { todo!() }
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