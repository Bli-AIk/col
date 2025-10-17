use crate::codegen;
use crate::parser::*;
use owo_colors::OwoColorize;

/// Handle output display operations
pub struct OutputHandler;

impl OutputHandler {
    /// Display the original source code
    pub fn display_original_code(content: &str) {
        println!();
        println!("----------Output----------");
        println!();
        println!("{}\n {}\n", "Original Code:".green(), content);
    }

    /// Display the parsed AST
    pub fn display_ast(program: &program::Program) {
        // Set to true for pretty-printing the AST
        let is_pretty_print_ast = false;
        let debug_str = if is_pretty_print_ast {
            format!("{:#?}", program)
        } else {
            format!("{:?}", program)
        };
        println!(
            "{}\n {}\n",
            "AST Parsed:".green(),
            crate::utils::colorize::colorize_brackets(&debug_str)
        );
    }

    /// Display symbol table
    pub fn display_symbol_table(root_scope: &visitor::symbol_table_builder::Scope) {
        let is_pretty_print_symbol_table = true;
        let symbol_table_debug_str = if is_pretty_print_symbol_table {
            format!("{:#?}", root_scope)
        } else {
            format!("{:?}", root_scope)
        };

        println!(
            "{}\n {}\n",
            "Symbol Table:".green(),
            crate::utils::colorize::colorize_brackets(&symbol_table_debug_str)
        );
    }

    /// Display the generated LLVM IR and save to file
    pub fn display_and_save_ir(ir_generator: &codegen::ir_generator::IRGenerator) {
        // Display generated IR
        println!("\n{}", "Generated LLVM IR:".green());
        let ir_string = ir_generator.get_module().print_to_string().to_string();
        println!("{}", ir_string);

        // Save IR to file
        crate::handler::file_handler::FileHandler::save_ir_to_file(&ir_string);
    }
}
