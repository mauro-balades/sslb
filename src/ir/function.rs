use crate::ir::value::Value;
use crate::ir::value::Type;
use crate::ir::basic_block::{BasicBlock};

pub struct Function {
    value: Value,

    name: String,
    args: Vec<Type>,
    blocks: Vec<BasicBlock>
}