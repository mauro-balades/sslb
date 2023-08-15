use crate::ir::values::function::Function;
use crate::targets::triple::TargetTriple;
use crate::targets::layout::DataLayout;
use std::fmt::Display;
use std::fmt::Formatter;

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
    globals: Vec<String> // just a simple lookup table
}

impl Module {
    /// Creates a new module builder.
    pub fn new(name: &str, data_layout: DataLayout, target_triple: TargetTriple) -> Self {
        Self {
            functions: Vec::new(),
            name: name.to_string(),
            globals: Vec::new(),
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
    pub fn get_functions(&self) -> &Vec<Function> {
        &self.functions
    }

    /// Adds a function to the module.
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    /// Returns the name a gloval value should use.
    pub fn get_global_value_name(&self, name: &str) -> String {
        if self.globals.contains(&name.to_string()) {
            let mut i = 0;
            loop {
                let new_name = format!("{}.{}", name, i);
                if !self.globals.contains(&new_name) {
                    return new_name;
                }
                i += 1;
            }
        } else {
            name.to_string()
        }
    }        
}

impl Display for Module {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(&format!("# ModuleID = '{}'\n", self.name));
        s.push_str(&format!("source_name = \"{}\"\n", self.name));
        s.push_str(&format!("target datalayout = \"{}\"\n", self.data_layout));
        s.push_str(&format!("target triple = \"{}\"\n", self.target_triple));
        s.push_str("\n");
        for function in &self.functions {
            s.push_str(&function.to_string());
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}
