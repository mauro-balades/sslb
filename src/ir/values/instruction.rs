use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::value::Type;
use std::string::ToString;
use crate::ir::values::value::ValueEntity;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstructionType {
    Add(Box<ValueEntity>, Box<ValueEntity>),
    Sub(Box<ValueEntity>, Box<ValueEntity>),
    Mul(Box<ValueEntity>, Box<ValueEntity>),
    Div(Box<ValueEntity>, Box<ValueEntity>),
    Rem(Box<ValueEntity>, Box<ValueEntity>),
    Shl(Box<ValueEntity>, Box<ValueEntity>),
    Shr(Box<ValueEntity>, Box<ValueEntity>),
    And(Box<ValueEntity>, Box<ValueEntity>),
    Or(Box<ValueEntity>, Box<ValueEntity>),
    Xor(Box<ValueEntity>, Box<ValueEntity>),
    Eq(Box<ValueEntity>, Box<ValueEntity>),
    Ne(Box<ValueEntity>, Box<ValueEntity>),
    Lt(Box<ValueEntity>, Box<ValueEntity>),
    Le(Box<ValueEntity>, Box<ValueEntity>),
    Gt(Box<ValueEntity>, Box<ValueEntity>),
    Ge(Box<ValueEntity>, Box<ValueEntity>),
    Neg(Box<ValueEntity>),
    Not(Box<ValueEntity>),
    Load(Box<ValueEntity>),
    Store(Box<ValueEntity>, Box<ValueEntity>),
    Call(Box<ValueEntity>, Vec<Box<ValueEntity>>),
    Return(Box<ValueEntity>),
    Branch(Box<ValueEntity>),
    BranchIf(Box<ValueEntity>, Box<ValueEntity>),
    Phi(Vec<(Box<ValueEntity>, Box<ValueEntity>)>),
    VoidReturn,
    Unreachable,

    ConstantInt32(i32),
    ConstantInt64(i64),
    ConstantBool(bool),
}

impl_for_value!(Instruction {
    instruction_type: InstructionType,
});

impl Instruction {
    pub fn new(ty: Type, instruction_type: InstructionType, name: Option<String>) -> Self {
        let val = Value::new(ty, name.unwrap_or("".to_string()));
        Self {
            instruction_type,
            value: val,
        }
    }

    pub fn get_type(&self) -> Type {
        self.value.get_type()
    }

    pub fn get_name(&self) -> String {
        // constant instructions don't have names
        // we return it's values
        match &self.instruction_type {
            InstructionType::ConstantInt32(a) => return a.to_string(),
            InstructionType::ConstantInt64(a) => return a.to_string(),
            InstructionType::ConstantBool(a) =>  return a.to_string(),
            _ => return self.value.get_name()
        }
    }

    pub fn instruction_type(&self) -> &InstructionType {
        &self.instruction_type
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
            InstructionType::Add(a, b) => format!("{} = add {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Sub(a, b) => format!("{} = sub {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Mul(a, b) => format!("{} = mul {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Div(a, b) => format!("{} = div {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Rem(a, b) => format!("{} = rem {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Shl(a, b) => format!("{} = shl {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Shr(a, b) => format!("{} = shr {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::And(a, b) => format!("{} = and {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Or(a, b) => format!("{} = or {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Xor(a, b) => format!("{} = xor {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Eq(a, b) => format!("{} = eq {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Ne(a, b) => format!("{} = ne {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Lt(a, b) => format!("{} = lt {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Le(a, b) => format!("{} = le {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Gt(a, b) => format!("{} = gt {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Ge(a, b) => format!("{} = ge {} {}, {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Neg(a) => format!("{} = neg {} {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref()),
            InstructionType::Not(a) => format!("{} = not {} {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref()),
            InstructionType::Load(a) => format!("{} = load {} {}", self.value.to_string(), a.get_type().to_string(), a.get_as_ref()),
            InstructionType::Store(a, b) => format!("store {} {}, {}", a.get_type().to_string(), a.get_as_ref(), b.get_as_ref()),
            InstructionType::Call(a, b) => {
                let mut args = String::new();
                for arg in b {
                    args.push_str(&format!("{}, ", arg.get_as_ref()));
                }
                format!("{} = call {} {}({})", self.value.to_string(), a.get_type().to_string(), a.get_as_ref(), args)
            },
            InstructionType::Return(a) => format!("return {} {}", a.get_type().to_string(), a.get_as_ref()),
            InstructionType::Branch(a) => format!("branch {}", a.get_as_ref()),
            InstructionType::BranchIf(a, b) => format!("branch {} {}", a.get_as_ref(), b.get_as_ref()),
            InstructionType::Phi(a) => {
                let mut args = String::new();
                for arg in a {
                    args.push_str(&format!("{}, ", arg.0.to_string()));
                    args.push_str(&format!("{}, ", arg.1.get_as_ref()));
                }
                format!("{} = phi {} {}", self.value.to_string(), a[0].0.get_type().to_string(), args)
            },
            InstructionType::Unreachable => format!("unreachable"),
            InstructionType::VoidReturn => format!("return void"),
            InstructionType::ConstantInt32(a) => format!("{}", a),
            InstructionType::ConstantInt64(a) => format!("{}", a),
            InstructionType::ConstantBool(a) => format!("{}", a),
        }
    }
}
