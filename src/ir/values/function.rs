use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use crate::ir::values::basic_block::{BasicBlock};

pub struct Function {
    value: Value,

    name: String,
    args: Vec<Type>,
    blocks: Vec<BasicBlock>
}