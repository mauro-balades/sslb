use crate::ir::values::function::Function;
use crate::ir::values::basic_block::BasicBlock;

pub struct Value {
    ty: Type,
    use_list: Vec<Value>,
    name: String,
}

pub enum ValueEntity {
    Function(Function),
    BasicBlock(BasicBlock),
}

pub enum Type {
    Integer(usize),
    Float(usize),
    FunctionType(/* arguments = */ Vec<Type>, /* return type = */ Box<Type>),
    Pointer(Box<Type>),
    Array(/* length = */ usize, /* element type = */ Box<Type>),
    Struct(/* field types = */ Vec<Type>),
    Void,
    Branch,
}

impl From<Function> for ValueEntity {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}

impl From<BasicBlock> for ValueEntity {
    fn from(basic_block: BasicBlock) -> Self {
        Self::BasicBlock(basic_block)
    }
}

impl Value {
    pub fn new(ty: Type, name: String) -> Self {
        Self {
            ty,
            use_list: Vec::new(),
            name,
        }
    }

    pub fn get_type(&self) -> &Type {
        &self.ty
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_use_list(&self) -> &Vec<Value> {
        &self.use_list
    }

    pub fn add_use(&mut self, value: Value) {
        self.use_list.push(value);
    }
}
