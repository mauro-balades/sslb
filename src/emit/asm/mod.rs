
use crate::ir::builder::ctx::IRContext;
use crate::targets::TargetTriple;
use crate::targets::triple::Arch;

use std::io::Write;

pub mod x86_64;
 
pub struct AssemblyEmitter {
    ctx: IRContext,
}

impl AssemblyEmitter {
    pub fn new(ctx: IRContext) -> Self {
        Self {
            ctx
        }
    }

    pub fn emit_module(&mut self, file: &mut impl Write) -> Result<(), std::io::Error> {
        match self.ctx.get_module().target_triple().arch() {
            Arch::X86_64 => x86_64::emit_module(self.ctx.clone(), file),
            _ => todo!()
        }
    }   
}