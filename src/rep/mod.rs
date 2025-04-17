pub mod ctl;
pub mod data;
pub mod scope;

use std::fmt::Debug;
use ctl::Start;
use data::Int;

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

pub trait Instr : Debug  {
    // default:
    fn peephole(self: Box<Self>, start: Start) -> Box<dyn Instr> where Self: Sized + 'static {
        let typ = self.eval_type();
        let instr: Box<dyn Instr> = match self.kind() {
            InstrKind::Int => self,
            _ => if typ.is_constant() { Box::new(Int::new(Box::new(start), typ))} else { self },
        };
        return instr;
    }
    fn eval_type(&self) -> TypeKind { TypeKind::Bot }
    fn add_input(&mut self, input: Box<dyn Instr>) -> () {}
    fn add_output(&mut self, input: Box<dyn Instr>) -> () {}

    // required:
    fn kind(&self) -> InstrKind;
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