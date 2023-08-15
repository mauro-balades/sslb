use crate::impl_for_value;
use crate::ir::values::value::Value;
use crate::ir::values::basic_block::{BasicBlock};
use crate::ir::linkage::Linkage;
use crate::ir::values::value::Type;

impl_for_value!(Function {
    blocks: Vec<BasicBlock>,
    params: Vec<Value>,

    is_var_arg: bool,

    linkage: Linkage,
});

impl Function {
    pub fn new(name: String, params: Vec<Value>, retTy: Type, is_var_arg: bool, linkage: Linkage) -> Self {
        let argTypes = params.iter().map(|param| param.get_type()).collect();
        let val = Value::new(Type::FunctionType(argTypes, Box::new(retTy)), name);
        Self {
            value: val,
            blocks: vec![],
            params,
            is_var_arg,
            linkage,
        }
    }

    pub fn create(name: String, ty: Type, linkage: Linkage) -> Self {
        assert!(ty.is_function_type());
        let val = Value::new(ty, name);
        Self {
            value: val,
            blocks: vec![],
            params: vec![],
            is_var_arg: false,
            linkage: linkage,
        }
    }

    pub fn is_external(&self) -> bool {
        self.linkage == Linkage::ExternalLinkage
    }

    pub fn is_var_arg(&self) -> bool {
        self.is_var_arg
    }

    pub fn get_name(&self) -> &str {
        &self.value.get_name()
    }

    pub fn get_block(&self, name: &str) -> Option<&BasicBlock> {
        self.blocks.iter().find(|block| block.get_name() == name)
    }

    pub fn get_block_mut(&mut self, name: &str) -> Option<&mut BasicBlock> {
        self.blocks.iter_mut().find(|block| block.get_name() == name)
    }

    pub fn get_param(&self, name: &str) -> Option<&Value> {
        self.params.iter().find(|param| param.get_name() == name)
    }

    pub fn get_param_mut(&mut self, name: &str) -> Option<&mut Value> {
        self.params.iter_mut().find(|param| param.get_name() == name)
    }

    pub fn get_param_by_index(&self, index: usize) -> Option<&Value> {
        self.params.get(index)
    }

    pub fn get_param_by_index_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.params.get_mut(index)
    }

    pub fn get_function_return_type(&self) -> Type {
        self.value.get_type().get_function_return_type()
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name() && other.value == self.value
    }
}
