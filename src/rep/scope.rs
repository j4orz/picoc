use std::{cell::RefCell, collections::{HashMap, VecDeque}, fmt::Debug, rc::{Rc, Weak}};
use thiserror::Error;
use super::{Instr, InstrKind, TypeKind};

#[derive(Error, Debug)]
pub enum ScopeError {
    #[error("double define")]
    DoubleDefine,
    #[error("unknown scope error")]
    Unknown,
}

// NB1: the program scope is a stack of nvs which are themselves maps from
//      aliases to usizes which are indices into its inputs. every instruction
//      node is dead on creation unless aliased by the scope node. once an
//      expression is aliased, it lives until the nv corresponding to the
//      lexical scope is popped by the caller and dropped by rust.
// NB2: each map in the stack from last to first corresponds to increasingly
//      nested lexical scopes. the language we are compiling for is sequential
//      not parallel so there can only be one active map for index i at any
//      given time. for example, in the snippet below, only one map will exist
//      at a time. if compiling for a language with parallel semantics,
//      the internal representation can shift to vecdeque<linkedlist<map<string, usize>>>
//      e.g
//      {
//        int a = 9;
//        int b = 10;
//        c = a+b;
//      }

//      {
//        int d = 11;
//        int e = 12;
//        f = d+e;
//      }

enum ScopeOp { Read, Update(Rc<dyn Instr>) }

#[derive(Debug, Clone)]
pub struct Scope { typ: TypeKind, inputs: RefCell<Vec<Rc<dyn Instr>>>, pub outputs: RefCell<Vec<Weak<dyn Instr>>>, nvs: RefCell<VecDeque<HashMap<String, usize>>> }
impl Scope {
    pub fn new() -> Self { Scope { typ: TypeKind::Bot, inputs: RefCell::new(vec![]), outputs: RefCell::new(vec![]), nvs: RefCell::new(VecDeque::new()) } }
    pub fn push_nv(self: Rc<Self>) -> () { self.nvs.borrow_mut().push_back(HashMap::new()); }
    pub fn pop_nv(self: Rc<Self>) -> () { self.nvs.borrow_mut().pop_back(); }
    pub fn read(self: Rc<Self>, alias: String) -> Rc<dyn Instr> { self.read_update(alias, ScopeOp::Read, self.nvs.borrow().len()-1) } // &mut self for shared foo. lazy phi.
    pub fn update(self: Rc<Self>, alias: String, expr: Rc<dyn Instr>) -> Rc<dyn Instr> { self.read_update(alias, ScopeOp::Update(expr), self.nvs.borrow().len()-1)}

    // shared read/update makes lazi phi creation easier TODO
    fn read_update(self: &Rc<Self>, alias: String, op: ScopeOp, level: usize) -> Rc<dyn Instr> {
        if level == 0 {
            todo!()
        }

        let nvs = self.nvs.borrow();
        let nv = nvs.get(level).unwrap();
        if let Some(i) = nv.get(&alias) {
            let expr = self.inputs.borrow()[*i].clone();
            match op {
                ScopeOp::Read => expr,
                ScopeOp::Update(instr) => {
                    self.inputs.borrow_mut()[*i] = instr; // updating std::vec calls drop on rc
                    self.inputs.borrow()[*i].clone() // rc.clone()
                },
            }
        } else {
            return self.read_update(alias, op, level-1);
        }
    }


    fn write(self: &Rc<Self>, alias: String, expr: Rc<dyn Instr>) -> Result<&Rc<Self>, ScopeError> {
        let mut nvs = self.nvs.borrow_mut();
        let cur_nv = nvs.back_mut().ok_or(ScopeError::Unknown)?;
        if cur_nv.contains_key(&alias) {
            Err(ScopeError::DoubleDefine)
        } else {
            let foo = self.add_child(expr);
            cur_nv.insert(alias, self.inputs.borrow().len());
            Ok(self)
        }
    }
}

impl Instr for Scope {
    fn kind(&self) -> InstrKind { InstrKind::Scope }
    fn inputs(&self) -> &RefCell<Vec<Rc<dyn Instr>>> { &self.inputs }
    fn outputs(&self) -> &RefCell<Vec<Weak<dyn Instr>>> { &self.outputs }
}