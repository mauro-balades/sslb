use std::io::Write;
use crate::ir::builder::ctx::IRContext;
use crate::ir::values::function::Function;
use crate::ir::linkage::Linkage;
use crate::ir::values::basic_block::BasicBlock;
use crate::ir::values::instruction::Instruction;
use crate::ir::values::value::ValueEntity;
use crate::ir::values::instruction::InstructionType;

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
        writeln!(file, "\t\t.text")?;
        writeln!(file, "\t\t.intel_syntax noprefix")?;

        let functions = self.ctx.get_module().get_functions().clone();
        for function in &functions {
            self.emit_function(file, &function.borrow(), true)?;
        }
        for function in functions {
            self.emit_function(file, &function.borrow(), false)?;
        }

        Ok(())
    }

    pub fn emit_function(&mut self, file: &mut impl Write, func: &Function, decl: bool) -> Result<(), std::io::Error> {
        if decl {
            // write the function prefix for linkage
            match func.get_linkage() {
                Linkage::ExternalLinkage => writeln!(file, "\t\t.extern {}", func.get_name())?,
                Linkage::InternalLinkage => writeln!(file, "\t\t.globl {}", func.get_name())?,
                Linkage::PrivateLinkage => writeln!(file, "")?,
                Linkage::ExternalWeakLinkage => writeln!(file, "\t\t.extern {}", func.get_name())?,
                Linkage::CommonLinkage => writeln!(file, "\t\t.extern {}", func.get_name())?,

                Linkage::AppendingLinkage => todo!(),
                Linkage::LinkonceLinkage => todo!(),
                Linkage::WeakLinkage => todo!(),
            }
            return Ok(());
        }

        if *func.get_linkage() == Linkage::ExternalLinkage {
            return Ok(());
        }

        // write the function name
        writeln!(file, "{}:", func.get_name())?;
        let blocks = func.get_blocks().clone();
        let mut i = 0;
        for block in blocks {
            self.emit_basic_block(file, block.borrow().clone(), func, i == 0)?;
            i += 1;
        }

        writeln!(file, "")?;
        Ok(())
    }

    pub fn emit_basic_block(&mut self, file: &mut impl Write, bb: BasicBlock, func: &Function, is_entry: bool) -> Result<(), std::io::Error> {
        if !is_entry {
            write!(file, ".{}.L{}:", func.get_name(), bb.get_name())?;
        }

        writeln!(file, "# %{}", bb.get_name())?;

        let insts = bb.get_instructions().clone();
        for inst in insts {
            self.emit_instruction(file, inst)?;
        }
        Ok(())
    }

    pub fn emit_instruction(&mut self, file: &mut impl Write, x: ValueEntity) -> Result<(), std::io::Error> {
        //match x {
        //    ValueEntity::Instruction(mut ref inst) => {
        //        match inst.get_type() {
        //            InstructionType::Add(x, y) => {
        //                writeln!(file, "  add rax, rax")?;
        //            }
        //    }
        //    _ => panic!("expected instruction")
        //}
        Ok(())
    }
}

pub fn emit_module(ctx: IRContext, file: &mut impl Write) -> Result<(), std::io::Error> {
    let mut emitter = X86_64Emitter::new(ctx);
    emitter.emit_module(file)
}
