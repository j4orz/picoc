pub mod ctl;
pub mod data;

use std::collections::{HashMap, VecDeque};
use ctl::{ReturnFields, StartFields};
use data::{AddFields, ConstantFields, DivFields, MulFields, NegFields, SubFields};


// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 {
    unsafe {
        ID += 1;
        ID
    }

}

// ******************** 1. SCOPE ********************
#[derive(Debug, Clone)]
pub struct ScopeFields { pub nvs: VecDeque<HashMap<String, i128>>, typ: Type }
impl ScopeFields {
    pub fn new() -> Self { ScopeFields { nvs: VecDeque::new(), typ: Type::Bot } }
    pub fn push_nv(&mut self) -> () { self.nvs.push_back(HashMap::new()); }
    pub fn pop_nv(&mut self) -> () { self.nvs.pop_back(); }

    fn var_def(&self) -> Instr {
        todo!()
    }

    fn var_apply(&self) -> Instr {
        todo!()
    }
}

// ******************** 2. STATICS(TYPES) ********************
// these types construct a lattice, which is a partially ordered set with unique
// least upper bounds and greatest lower bounds
// see: https://en.wikipedia.org/wiki/Lattice_(order)
#[derive(Debug, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Type { Bot, Top, Simple, Int(i128) }
impl Type {
    pub fn is_constant(&self) -> bool {
        match self {
            Type::Bot => false,
            Type::Top => true,
            Type::Simple => todo!(),
            Type::Int(_) => true,
        }
    }
}

// enforcing data via set/get methods on trait
trait TypedInstr {
    fn get_type(&self) -> Type;
}

impl TypedInstr for Instr {
    fn get_type(&self) -> Type {
        match self {
            Instr::Start(fields) => fields.typ,
            Instr::Return(fields) => fields.typ,
            Instr::Constant(fields) => fields.typ,
            Instr::Add(fields) => fields.typ,
            Instr::Sub(fields) => fields.typ,
            Instr::Mul(fields) => fields.typ,
            Instr::Div(fields) => fields.typ,
            Instr::Neg(fields) => fields.typ,
            Instr::Scope(_) => todo!("Scope doesn't have a type field yet"), // Or return a default/special type
        }
    }
}

// ******************** 2. DYNAMICS(VALUES) ********************
// NB1. the type for all instruction variant constructors (Self::new()) default
//      to Type::Bot since peephole optimizations are pessimistically monotonic
//      over the lattice.

// NB2. the scope (a stack of nv) neither data nor control is still represented
//      as an instruction within the graph to leverage def-use information in
//      liveness analysis. the scope instruction has no outputs. that is, no
//      instructions that use the scope instruction itself
#[derive(Debug, Clone)]
#[rustfmt::skip]
pub enum Instr {
    Scope(ScopeFields), // scope
    Start(StartFields), Return(ReturnFields), // control
    Constant(ConstantFields), Add(AddFields), Sub(SubFields), Mul(MulFields), Div(DivFields), Neg(NegFields), // data
}


