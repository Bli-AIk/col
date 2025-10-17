use codegen_handler::*;
use handler::*;
use output_handler::*;
use parse_handler::*;
use symbol_table_handler::*;

pub mod codegen;
pub mod ffi;
pub mod handler;
pub mod parser;
pub mod tests;
pub mod token;
pub mod utils; // Add FFI module

fn main() {
    let path = "ComplexTest.gml";

    // Read source file
    let content = match file_handler::FileHandler::read_source_file(path) {
        Ok(content) => content,
        Err(_) => return,
    };

    // Display original code
    OutputHandler::display_original_code(&content);

    // Perform lexical analysis
    ParseHandler::perform_lexical_analysis(&content);

    // Parse the source code
    let program = match ParseHandler::parse_source_code(&content) {
        Ok(program) => program,
        Err(_) => return,
    };

    // Build symbol table
    SymbolTableHandler::build_and_display_symbol_table(&program);

    // Generate LLVM IR and execute with JIT
    CodeGenHandler::generate_ir_and_execute(&program);
}
