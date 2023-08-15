use crate::ir::values::function::Function;
use crate::targets::triple::TargetTriple;
use crate::targets::layout::DataLayout;

/// A module builder for constructing Intermediate Representation (IR) modules.
///
/// This module builder offers a high-level interface for creating IR modules, which
/// serve as fundamental components of program representation. Modules encompass global
/// variables, functions, and their associated instructions.
///
/// By utilizing the module builder, you can define and organize global variables,
/// functions, and their corresponding instructions in a structured manner. Various
/// types of instructions, such as memory operations, control flow manipulations,
/// arithmetic computations, and more, can be added and managed seamlessly.
pub struct Module {
    functions: Vec<Function>,
    name: String,
    data_layout: DataLayout,
    target_triple: TargetTriple,
}

impl Module {
    /// Creates a new module builder.
    pub fn new(name: &str, data_layout: DataLayout, target_triple: TargetTriple) -> Self {
        Self {
            functions: Vec::new(),
            name: name.to_string(),
            data_layout,
            target_triple,
        }
    }

    /// Returns the name of the module.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the data layout of the module.
    pub fn data_layout(&self) -> &DataLayout {
        &self.data_layout
    }

    /// Returns the target triple of the module.
    pub fn target_triple(&self) -> &TargetTriple {
        &self.target_triple
    }

    /// Returns the functions of the module.
    pub fn functions(&self) -> &[Function] {
        &self.functions
    }
}
