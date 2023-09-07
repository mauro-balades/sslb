pub mod ctx;

use crate::ir::values::value::Value;
use crate::ir::values::basic_block::BasicBlock;
use crate::ir::values::value::ValueEntity;
use crate::ir::values::instruction::Instruction;
use crate::ir::values::instruction::InstructionType;
use crate::ir::values::value::Type;
use crate::ir::values::function::Function;
use crate::ir::linkage::Linkage;
use crate::utils::find_element;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::emit::asm::AssemblyEmitter;
use std::io::Write;

pub use crate::ir::builder::ctx::IRContext;

pub struct Builder {
    pub ctx: ctx::IRContext,
}

impl Builder {
    pub fn new(ctx: ctx::IRContext) -> Self {
        Self {
            ctx,
        }
    }

    pub fn get_module(&self) -> &crate::ir::module::Module {
        self.ctx.get_module()
    }

    pub fn set_insertion_point(&mut self, insertion_point: Rc<RefCell<BasicBlock>>) {
        self.ctx.insertion_point = Some(insertion_point);
    }

    pub fn insert(&mut self, value: Instruction) {
        // we can't insert to a non-existent insertion point
        match &mut self.ctx.insertion_point {
            Some(ref mut insertion_point) => {
                insertion_point.borrow_mut().insert(ValueEntity::Instruction(value));
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

    pub fn get_i8_ptr_type(&self) -> Type {
        self.get_pointer_type(self.get_i8_type())
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

    pub fn get_int_n_type(&self, bit_width: usize) -> Type {
        Type::Integer(bit_width)
    }

    pub fn get_branch_type(&self) -> Type {
        Type::Branch
    }

    // utility

    pub fn get_i32(&self, value: i32) -> ValueEntity {
        ValueEntity::Instruction(Instruction::new(self.get_i32_type(), InstructionType::ConstantInt32(value), None))
    }

    pub fn get_i64(&self, value: i64) -> ValueEntity {
        ValueEntity::Instruction(Instruction::new(self.get_i64_type(), InstructionType::ConstantInt64(value), None))
    }

    pub fn get_bool(&self, value: bool) -> ValueEntity {
        ValueEntity::Instruction(Instruction::new(self.get_bool_type(), InstructionType::ConstantBool(value), None))
    }

    pub fn create_function(&mut self, name: &str, argument_types: Vec<Type>, return_type: Type, linkage: Linkage, is_varg: bool) -> Rc<RefCell<Function>> {
        let fn_type = self.get_function_type(return_type, argument_types);
        let func = Rc::new(RefCell::new(Function::create(self.ctx.get_module().get_global_value_name(name), fn_type, linkage, is_varg)));
        self.ctx.get_module_mut().add_function(func.clone()); // TODO: add arguments
        func
    }

    pub fn get_block_inst_name(&mut self, name: Option<&str>) -> Option<String> {
        Some(match name {
            Some(name) => format!("%{}", name), // TODO: check if name is valid and if it should be wrapped in ""s
            None => format!("%{}", match self.ctx.insertion_point.as_ref() {
                Some(insertion_point) => insertion_point.borrow_mut().get_new_instruction_name(),
                None => panic!("Cannot get block instruction name without insertion point"),
            })
        }.to_string())
    }

    //pub fn get_or_create_function(&mut self, name: &str, argument_types: Vec<Type>, return_type: Type) -> &mut Function {
    //    let new_function = if let Some(existing_function) = self.ctx.get_module().get_functions().iter().find(|x| x.get_name() == name) {
    //        assert_eq!(existing_function.get_type(), self.get_function_type(return_type, argument_types));
    //        existing_function
    //    } else {
    //        self.add_extern_function(name, argument_types, return_type)
    //    };
    //
    //    // Ensure the lifetime of the returned reference doesn't exceed the borrow of `self`
    //    new_function
    //}
    

    // create instructions
    pub fn create_block(&mut self, name: &str, function: Rc<RefCell<Function>>) -> Rc<RefCell<BasicBlock>> { // TODO: name can be empty!
        let x = Rc::new(RefCell::new(BasicBlock::new(self.ctx.get_module().get_global_value_name(name), function.clone())));
        function.borrow_mut().add_block(x.clone());
        return x;
    }
    
    pub fn add(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Add(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn sub(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Sub(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn mul(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Mul(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn div(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Div(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn rem(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(lhs.get_type(), InstructionType::Rem(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn shl(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Shl(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn shr(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Shr(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn and(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(),InstructionType::And(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn or(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Or(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn xor(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer());
        let value = Instruction::new(lhs.get_type(), InstructionType::Xor(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn eq(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Eq(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn ne(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Ne(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn lt(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Lt(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn le(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Le(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn gt(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Gt(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn ge(&mut self, lhs: ValueEntity, rhs: ValueEntity, name: Option<&str>) -> Instruction {
        assert_eq!(lhs.get_type(), rhs.get_type());
        assert!(lhs.get_type().is_integer() || lhs.get_type().is_float());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Ge(Box::new(lhs), Box::new(rhs)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn neg(&mut self, value: ValueEntity, name: Option<&str>) -> Instruction {
        assert!(value.get_type().is_integer() || value.get_type().is_float());
        let value = Instruction::new(value.get_type(), InstructionType::Neg(Box::new(value)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn not(&mut self, value: ValueEntity, name: Option<&str>) -> Instruction {
        assert!(value.get_type().is_integer());
        let value = Instruction::new(self.get_bool_type(), InstructionType::Not(Box::new(value)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn load(&mut self, ty: Type, value: ValueEntity, name: Option<&str>) -> Instruction {
        assert!(value.get_type().is_pointer());
        let value = Instruction::new(ty, InstructionType::Load(Box::new(value)), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn store(&mut self, lhs: ValueEntity, rhs: ValueEntity) -> Instruction {
        assert!(lhs.get_type().is_pointer());
        assert_eq!(lhs.get_type().get_pointer_element_type(), rhs.get_type());
        let value = Instruction::new(self.get_void_type(), InstructionType::Store(Box::new(lhs), Box::new(rhs)), None);
        self.insert(value.clone());
        value
    }

    pub fn call(&mut self, callee: ValueEntity, args: Vec<ValueEntity>, name: Option<&str>) -> Instruction {
        assert!(callee.get_type().is_pointer());
        for (i, arg) in args.iter().enumerate() {
            assert_eq!(arg.get_type(), callee.get_type().get_pointer_element_type().get_function_argument_type(i as usize));
        }
        let boxedArgs = args.into_iter().map(|x| Box::new(x)).collect();
        let value = Instruction::new(callee.get_type().get_pointer_element_type().get_function_return_type(), InstructionType::Call(Box::new(callee), boxedArgs), self.get_block_inst_name(name));
        self.insert(value.clone());
        value
    }

    pub fn void_ret(&mut self) -> Instruction {
        assert_eq!(self.ctx.insertion_point.as_ref().unwrap().borrow_mut().get_parent().as_ref().borrow().get_type().get_function_return_type(), self.get_void_type(),
                   "Return value type does not match function return type (expected {:?}, got {:?})", self.ctx.insertion_point.as_ref().unwrap().borrow_mut().get_parent().as_ref().borrow().get_function_return_type(), self.get_void_type());
        let value = Instruction::new(self.get_void_type(), InstructionType::VoidReturn, None);
        self.insert(value.clone());
        value
    }

    pub fn ret(&mut self, value: ValueEntity) -> Instruction {
        assert_eq!(value.get_type(), self.ctx.insertion_point.as_ref().unwrap().borrow_mut().get_parent().as_ref().borrow().get_type().get_function_return_type(),
                   "Return value type does not match function return type (expected {:?}, got {:?})", self.ctx.insertion_point.clone().unwrap().as_ref().borrow().get_parent().as_ref().borrow().get_function_return_type(), value.get_type());
        let value = Instruction::new(self.get_void_type(), InstructionType::Return(Box::new(value)), None);
        self.insert(value.clone());
        value
    }

    pub fn branch(&mut self, target: ValueEntity) -> Instruction {
        assert!(target.get_type().is_branch());
        let value = Instruction::new(self.get_void_type(), InstructionType::Branch(Box::new(target)), None);
        self.insert(value.clone());
        value
    }

    pub fn branch_if(&mut self, condition: ValueEntity, target: ValueEntity) -> Instruction {
        assert!(condition.get_type().is_integer());
        assert!(target.get_type().is_branch());
        let value = Instruction::new(self.get_void_type(), InstructionType::BranchIf(Box::new(condition), Box::new(target)), None);
        self.insert(value.clone());
        value
    }

    pub fn phi(&mut self, incoming: Vec<(ValueEntity, ValueEntity)>) -> Instruction {
        assert!(incoming.iter().all(|(value, _)| value.get_type() == incoming[0].0.get_type()));
        let boxedIncoming = incoming.clone().into_iter().map(|(value, block)| (Box::new(value), Box::new(block))).collect();
        let value = Instruction::new(incoming[0].0.get_type(), InstructionType::Phi(boxedIncoming), None);
        self.insert(value.clone());
        value
    }

    pub fn unreachable(&mut self) -> Instruction {
        let value = Instruction::new(self.get_void_type(), InstructionType::Unreachable, None);
        self.insert(value.clone());
        value
    }

    // emitters

    pub fn emit_assembly(&self, file: &mut impl Write) -> Result<(), std::io::Error> {
        let mut emitter = AssemblyEmitter::new(self.ctx.clone());
        emitter.emit_module(file)
    }
}
