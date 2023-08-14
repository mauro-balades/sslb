mod ctx;

use crate::ir::values::value::Value;
use crate::ir::values::basic_block::BasicBlock;
use crate::ir::values::value::ValueEntity;

pub struct Builder {
    pub ctx: ctx::IRContext,
}

impl Builder {
    pub fn new(ctx: ctx::IRContext) -> Self {
        Self {
            ctx,
        }
    }

    pub fn set_insertion_point(&mut self, insertion_point: BasicBlock) {
        self.ctx.insertion_point = Some(insertion_point);
    }

    pub fn insert(&mut self, value: ValueEntity) {
        // we can't insert to a non-existent insertion point
        match &mut self.ctx.insertion_point {
            Some(ref mut insertion_point) => {
                insertion_point.insert(value);
            },
            None => {
                panic!("Cannot insert value without insertion point");
            },
        }
    }
}
