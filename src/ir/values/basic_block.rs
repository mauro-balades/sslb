use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use crate::ir::values::value::ValueEntity;
use crate::ir::values::function::Function;

impl_for_value!(BasicBlock {
    instructions: Vec<ValueEntity>,

    parent: Function,
});

impl BasicBlock {
    pub fn new(name: String, parent: Function) -> Self {
        let value = Value::new(Type::Branch, name);
        Self {
            value,
            instructions: Vec::new(),

            parent,
        }
    }

    pub fn insert(&mut self, instruction: ValueEntity) {
        self.instructions.push(instruction);
    }

    pub fn get_parent(&self) -> &Function {
        &self.parent
    }
}
