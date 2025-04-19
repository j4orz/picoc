pub mod ctl;
pub mod data;
pub mod scope;

use std::{cell::RefCell, fmt::Debug, rc::{Rc, Weak}};
use data::Int;

/// NB1: all instructions in ctl and data submodules use rc for indirection
///      even if reference counting is not needed (count=1) to keep all types
///      consistent. that is, the use of rc/weak for some nodes in the graph
///      colors the type of pointer for all nodes in the graph to keep types
///      consistent.
/// 
/// NB2: all instruction constructors that implement the instr trait must call
///      .init_outputs() — this is invariant is enforced by the human reviewer.
///      type state combined with field encapsulation can be used to enforce
///      callers outside the module, but does not enforce within the module for
///      library implementors

// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 { unsafe { ID += 1; ID } }

// since the generics with trait bounds get monomorphized (static polymorphism),
// trait objects (dynamic polymorphism) is used because sea of nodes
// heterogeneity needs dynamic dispatch. since trait objects only provide
// polymorphism on behavior and not data, required methods lift individual struct
// data (which have interior mutability) to shared trait behavior. a single instr
// enum with variant-specific data would need to do the opposite, and "lower"
// shared behavior. a heterogenous graph implemented in rust is nasty regardless
// of how you slice/dice it.

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeKind { Bot, Top, Simple, Int(i128) } // see: https://en.wikipedia.org/wiki/Lattice_(order)
pub enum InstrKind { Start, Return, Int, Add, Sub, Mul, Div, Scope }

pub trait Instr : Debug {
    fn kind(&self) -> InstrKind;
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>>;
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>>;
    fn eval_type(&self) -> TypeKind { TypeKind::Bot }

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
            InstrKind::Int => self,
            _ => if typ.is_constant() { Int::new(start.clone(), typ) } else { self },
        };
        return instr;
    } // NB1: self is Drop if instr no longer aliases it, possibly cascading to children
      // NB2: edge maintenance with weak pointers on on the children of the
      //      dropped instr are left for now which can be cleared eagerly
      //      (from parent's dealloc) or lazily (on child's processing) in the future
}

impl TypeKind {
    pub fn is_constant(&self) -> bool {
        match self {
            TypeKind::Bot => false,
            TypeKind::Top => true,
            TypeKind::Simple => todo!(),
            TypeKind::Int(_) => true,
        }
    }
}