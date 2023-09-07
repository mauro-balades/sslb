use crate::ir::values::basic_block::BasicBlock;
use crate::ir::module::Module;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct IRContext {
    pub insertion_point: Option<Rc<RefCell<BasicBlock>>>,
    module: Module,
}

impl IRContext {
    pub fn new(module: Module) -> Self {
        Self {
            insertion_point: None,
            module,
        }
    }

    pub fn set_insertion_point(&mut self, bb: Rc<RefCell<BasicBlock>>) {
        self.insertion_point = Some(bb);
    }

    pub fn clear_insertion_point(&mut self) {
        self.insertion_point = None;
    }

    pub fn get_module(&self) -> &Module {
        &self.module
    }

    pub fn get_module_mut(&mut self) -> &mut Module {
        &mut self.module
    }
}
