use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use std::string::ToString;

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

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match &self.instruction_type {
            InstructionType::Add(a, b) => format!("{} = add {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Sub(a, b) => format!("{} = sub {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Mul(a, b) => format!("{} = mul {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Div(a, b) => format!("{} = div {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Rem(a, b) => format!("{} = rem {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Shl(a, b) => format!("{} = shl {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Shr(a, b) => format!("{} = shr {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::And(a, b) => format!("{} = and {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Or(a, b) => format!("{} = or {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Xor(a, b) => format!("{} = xor {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Eq(a, b) => format!("{} = eq {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Ne(a, b) => format!("{} = ne {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Lt(a, b) => format!("{} = lt {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Le(a, b) => format!("{} = le {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Gt(a, b) => format!("{} = gt {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Ge(a, b) => format!("{} = ge {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Neg(a) => format!("{} = neg {} {}", self.value.to_string(), a.get_type().to_string(), a.to_string()),
            InstructionType::Not(a) => format!("{} = not {} {}", self.value.to_string(), a.get_type().to_string(), a.to_string()),
            InstructionType::Load(a) => format!("{} = load {} {}", self.value.to_string(), a.get_type().to_string(), a.to_string()),
            InstructionType::Store(a, b) => format!("store {} {}, {}", a.get_type().to_string(), a.to_string(), b.to_string()),
            InstructionType::Call(a, b) => {
                let mut args = String::new();
                for arg in b {
                    args.push_str(&format!("{}, ", arg.to_string()));
                }
                format!("{} = call {} {}({})", self.value.to_string(), a.get_type().to_string(), a.to_string(), args)
            },
            InstructionType::Return(a) => format!("return {} {}", a.get_type().to_string(), a.to_string()),
            InstructionType::Branch(a) => format!("branch {}", a.to_string()),
            InstructionType::BranchIf(a, b) => format!("branch {} {}", a.to_string(), b.to_string()),
            InstructionType::Phi(a) => {
                let mut args = String::new();
                for arg in a {
                    args.push_str(&format!("{}, ", arg.0.to_string()));
                    args.push_str(&format!("{}, ", arg.1.to_string()));
                }
                format!("{} = phi {} {}", self.value.to_string(), a[0].0.get_type().to_string(), args)
            },
            InstructionType::Unreachable => format!("unreachable"),
        }
    }
}
