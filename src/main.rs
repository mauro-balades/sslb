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

    let printf = builder.create_function("printf", vec![builder.get_i8_ptr_type()], builder.get_i32_type(), Linkage::ExternalLinkage, true);

    let main_fn = builder.create_function("main", Vec::<Type>::new(), builder.get_i32_type(), Linkage::InternalLinkage, false);
    let entry = builder.create_block("entry", main_fn.clone());

    builder.set_insertion_point(entry.clone());

    let add = builder.add(builder.get_i32(1), builder.get_i32(2), None);
    builder.ret(add.into());

    println!("{:}", builder.get_module());

    let mut test_file = std::fs::File::create("test.s").unwrap();
    if builder.emit_assembly(&mut test_file).is_err() {
        panic!("Failed to emit assembly");
    }
}
