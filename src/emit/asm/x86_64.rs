use std::io::Write;
use crate::ir::builder::ctx::IRContext;

struct X86_64Emitter {
    ctx: IRContext,
}

impl X86_64Emitter {
    pub fn new(ctx: IRContext) -> Self {
        Self {
            ctx
        }
    }

    pub fn emit_module(&mut self, file: &mut impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
}

pub fn emit_module(ctx: IRContext, file: &mut impl Write) -> Result<(), std::io::Error> {
    let mut emitter = X86_64Emitter::new(ctx);
    emitter.emit_module(file)
}
