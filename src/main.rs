use sslb::ir::builder::Builder;
use sslb::ir::builder::IRContext;
use sslb::ir::module::Module;
use sslb::targets::{DataLayout, TargetTriple};

pub fn main() {
    let triple = TargetTriple::from_host().unwrap();
    let layout = DataLayout::from_triple(&triple);

    let module = Module::new("test", layout.clone(), triple.clone());
    let context = IRContext::new(module);
    let mut builder = Builder::new(context);

    println!("{:?}", triple);
    println!("{:?}", layout);
}
