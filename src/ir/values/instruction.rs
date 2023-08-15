use crate::impl_for_value;
use crate::ir::values::value::Value;

pub enum InstructionType {

}

impl_for_value!(Instruction {
    pub instruction_type: InstructionType,
});
