use crate::ir::values::function::Function;
use crate::ir::values::basic_block::BasicBlock;
use crate::ir::values::instruction::Instruction;

use std::string::ToString;

#[derive(Debug, Clone, Eq)]
pub struct Value {
    ty: Type,
    use_list: Vec<Value>,
    name: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValueEntity {
    Function(Function),
    BasicBlock(BasicBlock),
    Instruction(Instruction),
}

#[derive(Debug, Clone, Eq)]
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

impl From<Instruction> for ValueEntity {
    fn from(instruction: Instruction) -> Self {
        Self::Instruction(instruction)
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

    pub fn get_type(&self) -> Type {
        self.ty.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_use_list(&self) -> &Vec<Value> {
        &self.use_list
    }

    pub fn add_use(&mut self, value: Value) {
        self.use_list.push(value);
    }
}

impl Type {
    pub fn is_integer(&self) -> bool {
        match self {
            Type::Integer(_) => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match self {
            Type::Float(_) => true,
            _ => false,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            Type::FunctionType(_, _) => true,
            _ => false,
        }
    }

    pub fn is_pointer(&self) -> bool {
        match self {
            Type::Pointer(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            Type::Array(_, _) => true,
            _ => false,
        }
    }

    pub fn is_struct(&self) -> bool {
        match self {
            Type::Struct(_) => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Type::Void => true,
            _ => false,
        }
    }

    pub fn is_branch(&self) -> bool {
        match self {
            Type::Branch => true,
            _ => false,
        }
    }

    pub fn get_pointer_element_type(&self) -> Type {
        match self {
            Type::Pointer(ty) => (**ty).clone(),
            _ => panic!("not a pointer type"),
        }
    }

    pub fn get_function_return_type(&self) -> Type {
        match self {
            Type::FunctionType(_, ty) => (**ty).clone(),
            _ => panic!("not a function type"),
        }
    }

    pub fn get_function_argument_types(&self) -> &Vec<Type> {
        match self {
            Type::FunctionType(tys, _) => tys,
            _ => panic!("not a function type"),
        }
    }

    pub fn get_function_argument_type(&self, index: usize) -> Type {
        match self {
            Type::FunctionType(tys, _) => tys[index].clone(),
            _ => panic!("not a function type"),
        }
    }

    pub fn get_pointer_to(&self) -> Type {
        Type::Pointer(Box::new(self.clone()))
    }

    pub fn is_function_type(&self) -> bool {
        match self {
            Type::FunctionType(_, _) => true,
            _ => false,
        }
    }
}

#[macro_export]
macro_rules! impl_for_value {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, Eq)] // ewww
        pub struct $name {
            value: Value,

            $($field: $t),*
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Integer(size) => format!("i{}", size),
            Type::Float(size) => format!("f{}", size),
            Type::FunctionType(args, ret) => {
                let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(", ");
                format!("{} -> {}", args, ret.to_string())
            }
            Type::Pointer(ty) => format!("{}*", ty.to_string()),
            Type::Array(len, ty) => format!("[{} x {}]", len, ty.to_string()),
            Type::Struct(tys) => {
                let tys = tys.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join(", ");
                format!("{{ {} }}", tys)
            }
            Type::Void => "void".to_string(),
            Type::Branch => "branch".to_string(),
        }
    }
}

impl ToString for ValueEntity {
    fn to_string(&self) -> String {
        match self {
            ValueEntity::Function(function) => function.to_string(),
            ValueEntity::BasicBlock(basic_block) => basic_block.to_string(),
            ValueEntity::Instruction(instruction) => instruction.to_string(),
        }
    }
}

impl ValueEntity {
    pub fn get_type(&self) -> Type {
        match self {
            ValueEntity::Function(function) => function.get_type(),
            ValueEntity::BasicBlock(basic_block) => basic_block.get_type(),
            ValueEntity::Instruction(instruction) => instruction.get_type(),
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            ValueEntity::Function(function) => function.get_name(),
            ValueEntity::BasicBlock(basic_block) => basic_block.get_name(),
            ValueEntity::Instruction(instruction) => instruction.get_name(),
        }
    }

    pub fn get_as_ref(&self) -> String {
        self.get_name().to_string()
    }
}

// equality overloads

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Type::Integer(x), Type::Integer(y)) => x == y,
            (Type::Float(x), Type::Float(y)) => x == y,
            (Type::Pointer(x), Type::Pointer(y)) => x == y,
            (Type::Array(x1, x2), Type::Array(y1, y2)) => x1 == y1 && x2 == y2,
            (Type::Struct(x), Type::Struct(y)) => x == y,
            (Type::Void, Type::Void) => true,
            (Type::Branch, Type::Branch) => true,
            (Type::FunctionType(x1, x2), Type::FunctionType(y1, y2)) => x1 == y1 && x2 == y2,
            _ => false,
        }
    }
}