use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::basic_block::{BasicBlock};

impl_for_value!(Function {
    name: String,
    blocks: Vec<BasicBlock>,
    params: Vec<Value>,

    is_external: bool,
    is_var_arg: bool,
});