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

    pub fn get_type(&self) -> Type {
        self.value.get_type()
    }

    pub fn get_name(&self) -> &String {
        self.value.get_name()
    }

    pub fn get_instructions(&self) -> &Vec<ValueEntity> {
        &self.instructions
    }

    pub fn get_instructions_mut(&mut self) -> &mut Vec<ValueEntity> {
        &mut self.instructions
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut Value {
        &mut self.value
    }
}

impl PartialEq for BasicBlock {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
