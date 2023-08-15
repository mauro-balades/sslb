use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::value::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionType {
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Rem(Value, Value),
    Shl(Value, Value),
    Shr(Value, Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Eq(Value, Value),
    Ne(Value, Value),
    Lt(Value, Value),
    Le(Value, Value),
    Gt(Value, Value),
    Ge(Value, Value),
    Neg(Value),
    Not(Value),
    Load(Value),
    Store(Value, Value),
    Call(Value, Vec<Value>),
    Return(Value),
    Branch(Value),
    BranchIf(Value, Value),
    Phi(Vec<(Value, Value)>),
    Unreachable,
}

impl_for_value!(Instruction {
    instruction_type: InstructionType,
});

impl Instruction {
    pub fn new(ty: Type, instruction_type: InstructionType) -> Self {
        let val = Value::new(ty, "".to_string());
        Self {
            instruction_type,
            value: val,
        }
    }
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.instruction_type == other.instruction_type
    }
}
