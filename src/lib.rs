// Library interface for COL runtime
// This module exports the core functionality for embedding COL in other applications

pub mod codegen;
pub mod ffi;
pub mod handler;
pub mod parser;
pub mod tests;
pub mod token;
pub mod utils;

// Re-export key types for library users
pub use ffi::{
    COLResult, COLScript, COLValue, COLVariant, PrintCallback, col_call_function, col_compile_script,
    col_destroy_script, col_free_string, col_get_global_variable, col_get_last_error,
    col_get_script_error, col_initialize, col_print, col_print_boolean, col_print_number,
    col_register_print_callback, col_set_global_variable, col_shutdown,
};

pub use parser::{expr::Expr, program::Program, stmt::Stmt};

pub use handler::{codegen_handler::CodeGenHandler, parse_handler::ParseHandler};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
