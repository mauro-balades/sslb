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
    let tr = builder.create_block("if_true", main_fn.clone());
    let fs = builder.create_block("if_false", main_fn.clone());
    let cn = builder.create_block("if_cont", main_fn.clone());

    builder.set_insertion_point(entry.clone());

    let add = builder.add(builder.get_i32(1), builder.get_i32(2), None);
    let add2 = builder.eq(add.into(), builder.get_i32(2), None);
    
    builder.branch_if(add2.clone().into(), tr.clone().into(), fs.clone().into());

    builder.set_insertion_point(tr.clone());
    builder.ret(builder.get_i32(1).into());

    builder.set_insertion_point(fs.clone());
    builder.ret(builder.get_i32(0).into());

    builder.set_insertion_point(cn.clone());
    builder.ret(add2.clone().into());

    println!("{:}", builder.get_module());

    let mut test_file = std::fs::File::create("test.s").unwrap();
    if builder.emit_assembly(&mut test_file).is_err() {
        panic!("Failed to emit assembly");
    }
}
