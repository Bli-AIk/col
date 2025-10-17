use crate::parser::*;

/// Handle symbol table building
pub struct SymbolTableHandler;

impl SymbolTableHandler {
    /// Build symbol table and display it
    pub fn build_and_display_symbol_table(program: &program::Program) {
        let mut root_scope = visitor::symbol_table_builder::Scope::new();
        let mut builder = visitor::symbol_table_builder::SymbolTableBuilder::new(&mut root_scope);
        program.accept(&mut builder);

        crate::handler::output_handler::OutputHandler::display_symbol_table(&root_scope);
    }
}
