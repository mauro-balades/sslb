use crate::ir::values::basic_block::BasicBlock;
use crate::ir::module::Module;

pub struct IRContext {
    pub insertion_point: Option<BasicBlock>,
    pub module: Module,
}
