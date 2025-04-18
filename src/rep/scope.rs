use std::{collections::{HashMap, VecDeque}, fmt::Debug, rc::Rc, sync::Arc};
use thiserror::Error;

use super::{Instr, TypeKind};

#[derive(Error, Debug)]
pub enum FooError {
    #[error("broadcast mismatch")]
    BroadcastMismatch,
    #[error("double define")]
    DoubleDefine,
    #[error("unknown scope error")]
    Unknown,
}


#[derive(Debug, Clone)]
pub struct Scope { typ: TypeKind, inputs: Vec<Rc<Box<dyn Instr>>>, outputs: Vec<Rc<Box<dyn Instr>>>, pub nvs: VecDeque<HashMap<String, usize>> }
impl Scope {
    pub fn new() -> Self { Scope { typ: TypeKind::Bot, inputs: vec![], outputs: vec![], nvs: VecDeque::new() } }
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

// NB: the scope (a stack of nv) neither data nor control is still represented
//     as an instruction within the graph to leverage def-use information in
//     liveness analysis. the scope instruction has no outputs. that is, no
//     instructions that use the scope instruction itself
// impl Instr for Scope {
//     fn kind(&self) -> InstrKind { InstrKind::Scope }
// }