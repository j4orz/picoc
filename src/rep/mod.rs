pub mod ctl;
pub mod data;

use std::{collections::{HashMap, VecDeque}, fmt::Debug, rc::Rc, sync::Arc};
use thiserror::Error;

// FIXME: no static mut
static mut ID: i128 = 0;
pub fn fresh_id() -> i128 { unsafe { ID += 1; ID } }

#[derive(Error, Debug)]
pub enum FooError {
    #[error("broadcast mismatch")]
    BroadcastMismatch,
    #[error("double define")]
    DoubleDefine,
    #[error("unknown scope error")]
    Unknown,
}

// ******************** 1. SCOPE ********************
#[derive(Debug, Clone)]
pub struct Scope { typ: Type, inputs: Vec<Rc<Box<dyn Instr>>>, outputs: Vec<Rc<Box<dyn Instr>>>, pub nvs: VecDeque<HashMap<String, usize>> }
impl Scope {
    pub fn new() -> Self { Scope { typ: Type::Bot, inputs: todo!(), outputs: todo!(), nvs: VecDeque::new(), } }
    pub fn push_nv(&mut self) -> () { self.nvs.push_back(HashMap::new()); }
    pub fn pop_nv(&mut self) -> () { self.nvs.pop_back(); }

    fn var_def(&mut self, alias: String, expr: Box<dyn Instr>) -> Result<&mut Self, FooError> {
        let cur_nv = self.nvs.back_mut().ok_or(FooError::Unknown)?;
        if cur_nv.contains_key(&alias) {
            Err(FooError::DoubleDefine)
        } else {
            cur_nv.insert(alias, self.inputs.len());
            // self.add_input(expr);
            // expr.add_output(self);
            Ok(self)
        }
    }

    fn var_apply(&self) -> Arc<dyn Instr> {
        todo!()
    }
}

// NB2. the scope (a stack of nv) neither data nor control is still represented
//      as an instruction within the graph to leverage def-use information in
//      liveness analysis. the scope instruction has no outputs. that is, no
//      instructions that use the scope instruction itself
impl Instr for Scope {
    fn add_input(&mut self, input: Box<dyn Instr>) -> () {
        todo!()
    }

    fn add_output(&mut self, input: Box<dyn Instr>) -> () {
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

// ******************** 2. DYNAMICS(VALUES) ********************
// trait objects are used over generics and trait bounds because instructions in
// sea of nodes are heteregenous, since the latter gets monomorphized with one
// single type at compilation time.
pub trait Instr : Debug  {
    fn add_input(&mut self, input: Box<dyn Instr>) -> ();
    fn add_output(&mut self, input: Box<dyn Instr>) -> ();
}

// NB1. the type for all instruction variant constructors (Self::new()) default
//      to Type::Bot since peephole optimizations are pessimistically monotonic
//      over the lattice.
// #[derive(Debug, Clone)]
// #[rustfmt::skip]
// pub enum InstrFoo {
//     Scope(ScopeFields), // scope
//     Start(Start), Return(ReturnFields), // control
//     Constant(ConstantFields), Add(AddFields), Sub(SubFields), Mul(MulFields), Div(DivFields), Neg(NegFields), // data
// }