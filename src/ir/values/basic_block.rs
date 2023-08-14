use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use crate::ir::values::function::Function;

pub struct BasicBlock {
    value: Value,
    instructions: Vec<Value>,

    parent: Function,
}

impl BasicBlock {
    pub fn new(name: String, parent: Function) -> Self {
        let value = Value::new(Type::Branch, name);
        Self {
            value,
            instructions: Vec::new(),

            parent,
        }
    }

    pub fn insert(&mut self, instruction: Value) {
        self.instructions.push(instruction);
    }

    pub fn get_parent(&self) -> &Function {
        &self.parent
    }
}
