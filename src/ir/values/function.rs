use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use crate::ir::values::basic_block::{BasicBlock};

pub struct Function {
    value: Value,

    name: String,
    blocks: Vec<BasicBlock>,
    params: Vec<Value>,

    is_external: bool,
    is_var_arg: bool,
}