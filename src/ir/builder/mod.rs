mod ctx;

use crate::ir::values::value::Value;
use crate::ir::values::basic_block::BasicBlock;
use crate::ir::values::value::ValueEntity;
use crate::ir::values::instruction::Instruction;
use crate::ir::values::instruction::InstructionType;
use crate::ir::values::value::Type;

pub struct Builder {
    pub ctx: ctx::IRContext,
}

impl Builder {
    pub fn new(ctx: ctx::IRContext) -> Self {
        Self {
            ctx,
        }
    }

    pub fn set_insertion_point(&mut self, insertion_point: BasicBlock) {
        self.ctx.insertion_point = Some(insertion_point);
    }

    pub fn insert(&mut self, value: Instruction) {
        // we can't insert to a non-existent insertion point
        match &mut self.ctx.insertion_point {
            Some(ref mut insertion_point) => {
                insertion_point.insert(ValueEntity::Instruction(value));
            },
            None => {
                panic!("Cannot insert value without insertion point");
            },
        }
    }
    
    // get types
    pub fn get_void_type(&self) -> Type {
        Type::Void
    }

    pub fn get_bool_type(&self) -> Type {
        Type::Integer(1)
    }

    pub fn get_i8_type(&self) -> Type {
        Type::Integer(8)
    }

    pub fn get_i16_type(&self) -> Type {
        Type::Integer(16)
    }

    pub fn get_i32_type(&self) -> Type {
        Type::Integer(32)
    }

    pub fn get_i64_type(&self) -> Type {
        Type::Integer(64)
    }

    pub fn get_i128_type(&self) -> Type {
        Type::Integer(128)
    }

    pub fn get_f32_type(&self) -> Type {
        Type::Float(32)
    }

    pub fn get_f64_type(&self) -> Type {
        Type::Float(64)
    }

    pub fn get_pointer_type(&self, element_type: Type) -> Type {
        element_type.get_pointer_to()
    }

    pub fn get_array_type(&self, element_type: Type, length: usize) -> Type {
        Type::Array(length, Box::new(element_type))
    }

    pub fn get_function_type(&self, return_type: Type, argument_types: Vec<Type>) -> Type {
        Type::FunctionType(argument_types, Box::new(return_type))
    }

    pub fn get_iN_type(&self, bit_width: usize) -> Type {
        Type::Integer(bit_width)
    }

    pub fn get_branch_type(&self) -> Type {
        Type::Branch
    }

    // create instructions

    pub fn add(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Add(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn sub(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Sub(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn mul(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Mul(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn div(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Div(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn rem(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Rem(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn shl(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Shl(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn shr(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Shr(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn and(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(),InstructionType::And(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn or(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Or(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn xor(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Xor(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn eq(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Eq(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn ne(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Ne(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn lt(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Lt(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn le(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Le(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn gt(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Gt(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn ge(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Ge(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn neg(&mut self, value: Value) -> Instruction {
        assert!(value.get_type().is_integer() || value.get_type().is_float());
        let value = Instruction::new(value.get_type(), InstructionType::Neg(value));
        self.insert(value.clone());
        value
    }

    pub fn not(&mut self, value: Value) -> Instruction {
        assert!(value.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Not(value));
        self.insert(value.clone());
        value
    }

    pub fn load(&mut self, ty: Type, value: Value) -> Instruction {
        assert!(value.get_type().is_pointer());
        let value = Instruction::new(ty, InstructionType::Load(value));
        self.insert(value.clone());
        value
    }

    pub fn store(&mut self, lhs: Value, rhs: Value) -> Instruction {
        assert!(lhs.get_type().is_pointer());
        assert_eq!(lhs.get_type().get_pointer_element_type(), rhs.get_type());
        let value = Instruction::new(self.get_void_type(), InstructionType::Store(lhs, rhs));
        self.insert(value.clone());
        value
    }

    pub fn call(&mut self, callee: Value, args: Vec<Value>) -> Instruction {
        assert!(callee.get_type().is_pointer());
        for (i, arg) in args.iter().enumerate() {
            assert_eq!(arg.get_type(), callee.get_type().get_pointer_element_type().get_function_argument_type(i as usize));
        }
        let value = Instruction::new(callee.get_type().get_pointer_element_type().get_function_return_type(), InstructionType::Call(callee, args));
        self.insert(value.clone());
        value
    }

    pub fn ret(&mut self, value: Value) -> Instruction {
        assert_eq!(value.get_type(), self.ctx.insertion_point.as_mut().unwrap().get_type().get_function_return_type(),
                   "Return value type does not match function return type (expected {:?}, got {:?})", self.ctx.insertion_point.as_mut().unwrap().get_parent().get_function_return_type(), value.get_type());
        let value = Instruction::new(self.get_void_type(), InstructionType::Return(value));
        self.insert(value.clone());
        value
    }

    pub fn branch(&mut self, target: Value) -> Instruction {
        assert!(target.get_type().is_branch());
        let value = Instruction::new(self.get_void_type(), InstructionType::Branch(target));
        self.insert(value.clone());
        value
    }

    pub fn branch_if(&mut self, condition: Value, target: Value) -> Instruction {
        assert!(condition.get_type().is_integer());
        assert!(target.get_type().is_branch());
        let value = Instruction::new(self.get_void_type(), InstructionType::BranchIf(condition, target));
        self.insert(value.clone());
        value
    }

    pub fn phi(&mut self, incoming: Vec<(Value, Value)>) -> Instruction {
        assert!(incoming.iter().all(|(value, _)| value.get_type() == incoming[0].0.get_type()));
        let value = Instruction::new(incoming[0].0.get_type(), InstructionType::Phi(incoming));
        self.insert(value.clone());
        value
    }

    pub fn unreachable(&mut self) -> Instruction {
        let value = Instruction::new(self.get_void_type(), InstructionType::Unreachable);
        self.insert(value.clone());
        value
    }
}
