use crate::ir::values::value::Value;

pub enum InstructionType {

}

pub struct Instruction {
    pub instruction_type: InstructionType,
    value: Value,
}
