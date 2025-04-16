// ************************************ DATA ***********************************

use std::sync::Arc;

use super::{fresh_id, Instr, Type};

#[rustfmt::skip] 
#[derive(Clone, Debug)]
pub struct ConstantFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Instr> }
impl ConstantFields {
    pub fn new(ctrl: Arc<Instr>, typ: Type) -> Self {
        Self {
            id: fresh_id(),
            typ,
            ud: vec![ctrl], // phantom edge to start enabling graph traversal
            du: vec![],
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct AddFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, pub x: Arc<Instr>, pub y: Arc<Instr> }
impl AddFields {
    pub fn new(x: Instr, y: Instr) -> Self {
        let ud = vec![Arc::new(x), Arc::new(y)];
        let (x, y) = (ud[0].clone(), ud[1].clone());
        Self { id: fresh_id(), typ: Type::Bot, ud, du: vec![], x, y }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct SubFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, x: Arc<Instr>, y: Arc<Instr> }
impl SubFields {
    pub fn new(x: Instr, y: Instr) -> Self {
        let ud = vec![Arc::new(x), Arc::new(y)];
        let (x, y) = (ud[0].clone(), ud[1].clone());
        Self { id: fresh_id(), typ: todo!(), ud, du: todo!(), x, y }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct MulFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, x: Arc<Instr>, y: Arc<Instr> }
impl MulFields {
    pub fn new(x: Instr, y: Instr) -> Self {
        let ud = vec![Arc::new(x), Arc::new(y)];
        let (x, y) = (ud[0].clone(), ud[1].clone());
        Self { id: fresh_id(), typ: todo!(), ud, du: todo!(), x, y }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct DivFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, x: Arc<Instr>, y: Arc<Instr> }
impl DivFields {
    pub fn new(x: Instr, y: Instr) -> Self {
        let ud = vec![Arc::new(x), Arc::new(y)];
        let (x, y) = (ud[0].clone(), ud[1].clone());
        Self { id: fresh_id(), typ: todo!(), ud, du: todo!(), x, y }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone)]
pub struct NegFields { id: i128, pub typ: Type, ud: Vec<Arc<Instr>>, du: Vec<Arc<Instr>>, x: Arc<Instr>, y: Arc<Instr> }
impl NegFields {
    pub fn new(x: Instr, y: Instr) -> Self {
        let ud = vec![Arc::new(x), Arc::new(y)];
        let (x, y) = (ud[0].clone(), ud[1].clone());
        Self { id: fresh_id(), typ: todo!(), ud, du: todo!(), x, y }
    }
}