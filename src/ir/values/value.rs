
pub struct Value {
    ty: Type,
    use_list: Vec<Value>,
    name: String,
}

pub enum Type {
    Integer(usize),
    Float(usize),
    FunctionType(/* arguments = */ Vec<Type>, /* return type = */ Type),
    Pointer(Box<Type>),
    Array(/* length = */ usize, /* element type = */ Box<Type>),
    Struct(/* field types = */ Vec<Type>),
    Void,
}
