
pub struct Value {
    ty: Type,
    use_list: Vec<Value>,
    name: String,
}

pub enum Type {
    Integer(usize),
    Float(usize),
    FunctionType(/* arguments = */ Vec<Type>, /* return type = */ Type),
}
