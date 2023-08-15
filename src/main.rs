

pub fn main() {
    let triple = targets::Tripple::from_host();
    let layout = targets::Layout::from_tripple(&tripple);

    let module = ir::Module::new("test", layout, triple);
    let context = ir::IRContext::new(module);
    let mut builder = ir::Builder::new();
}
