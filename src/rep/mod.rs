pub mod ctl;
pub mod data;
pub mod scope;

use std::{fmt::Debug, rc::Rc, cell::RefCell};
use data::Int;

/// NB1: all instructions in ctl and data submodules use rc for indirection
///      even if reference counting is not needed (count=1) to keep all
///      types consistent. that is, rc colors the type of pointer.
/// 
/// NB2: all instruction constructors that implement the instr trait must call
///      .init_outputs() — this is invariant is enforced by the human reviewer.
///      type state combined with field encapsulation can be used to enforce
///      callers outside the module, but does not enforce within the module for
///      library implementors

// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 { unsafe { ID += 1; ID } }

// trait objects (dynamic polymorphism) are used over generics and trait bounds
// because instructions in sea of nodes are heteregenous, since the latter gets
// monomorphized (static polymorphism) with one single type at compilation time.
// NB: trait objects only provide polymorphism on behavior, not data. so methods
//     with return TypeKind (statics) and InstrKind (dynamics) return types are
//     declared

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TypeKind { Bot, Top, Simple, Int(i128) } // see: https://en.wikipedia.org/wiki/Lattice_(order)
pub enum InstrKind { Start, Return, Int, Add, Sub, Mul, Div, Scope }

pub trait Instr : Debug {
    // type State<T> where T: 

    // accessors
    fn kind(&self) -> InstrKind;
    fn inputs(&self) -> &Vec<Rc<dyn Instr>>;
    fn outputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>>;
    fn fill_dus(self: &Rc<Self>) where Self: Sized + 'static {
        for i in self.inputs() {
            i.outputs().borrow_mut().push(self.clone() as Rc<dyn Instr>);
        }
    }

    // optimizer
    fn peephole(self: Rc<Self>, start: Rc<dyn Instr>) -> Rc<dyn Instr> where Self: Sized + 'static {
        let typ = self.eval_type();
        let instr: Rc<dyn Instr> = match self.kind() {
            InstrKind::Int => self,
            _ => if typ.is_constant() { Int::new(start.clone(), typ) } else { self },
        };
        return instr;
    }
    fn eval_type(&self) -> TypeKind { TypeKind::Bot }
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