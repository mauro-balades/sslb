use crate::ir::function::Function;
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