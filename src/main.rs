use sslb::ir::builder::Builder;
use sslb::ir::builder::IRContext;
use sslb::ir::module::Module;
use sslb::targets::{DataLayout, TargetTriple};
use sslb::ir::linkage::Linkage;
use sslb::ir::values::value::Type;

pub fn main() {
    let triple = TargetTriple::from_host().unwrap();
    let layout = DataLayout::from_triple(&triple);

    let module = Module::new("test", layout.clone(), triple.clone());
    let context = IRContext::new(module);
    let mut builder = Builder::new(context);

    let mut main_fn = builder.add_function("main", Vec::<Type>::new(), builder.get_void_type(), Linkage::InternalLinkage);
    let entry = builder.create_block("entry", &mut main_fn);
    

    println!("{:}", builder.get_module());
}
