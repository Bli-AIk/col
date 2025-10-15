use crate::codegen::TypeMapping;
use crate::parser::visitor::Visitor;
use crate::parser::{
    expr::Expr, func::Func, func_def::FuncDef, program::Program, stmt::Stmt, top_level::TopLevel,
};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::*;
use inkwell::values::*;
use std::collections::HashMap;

/// Error types for IR generation
#[derive(Debug)]
pub enum IRGenError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMismatch(String),
    InvalidOperation(String),
}

pub type IRGenResult<T> = Result<T, IRGenError>;

/// IR Generator that implements the Visitor pattern to generate LLVM IR
pub struct IRGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub type_mapping: TypeMapping<'ctx>,

    // Symbol tables
    pub(crate) variables: HashMap<String, PointerValue<'ctx>>,
    pub(crate) variable_types: HashMap<String, BasicTypeEnum<'ctx>>,
    pub(crate) functions: HashMap<String, FunctionValue<'ctx>>,

    // Current function context
    pub(crate) current_function: Option<FunctionValue<'ctx>>,
}

impl<'ctx> IRGenerator<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let type_mapping = TypeMapping::new(context);

        Self {
            context,
            module,
            builder,
            type_mapping,
            variables: HashMap::new(),
            variable_types: HashMap::new(),
            functions: HashMap::new(),
            current_function: None,
        }
    }

    /// Enter a function context
    pub fn enter_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
        // Create entry block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);

        // Clear local variables when entering new function
        self.variables.clear();
        self.variable_types.clear();
    }

    /// Exit function context
    pub fn exit_function(&mut self) {
        self.current_function = None;
        // Clear local variables when exiting function
        self.variables.clear();
        self.variable_types.clear();
    }
}

impl<'ctx> Visitor<IRGenResult<BasicValueEnum<'ctx>>> for IRGenerator<'ctx> {
    fn visit_program(&mut self, program: &Program) -> IRGenResult<BasicValueEnum<'ctx>> {
        // Create a main function to hold global statements
        let return_type = self.type_mapping.get_number_type();
        let fn_type = return_type.fn_type(&[], false);
        let main_function = self.module.add_function("main", fn_type, None);
        self.enter_function(main_function);

        let mut _last_value = self.gen_number_const(0.0).into();
        for top_level in &program.body {
            _last_value = self.visit_toplevel(top_level)?;
        }

        // Only add return if the block doesn't have a terminator
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                // Always return a double 0.0 from main function, regardless of last expression type
                let return_value = self.gen_number_const(0.0);
                self.builder
                    .build_return(Some(&return_value))
                    .map_err(|e| {
                        IRGenError::InvalidOperation(format!("Failed to build return: {}", e))
                    })?;
            }
        }

        self.exit_function();

        // Return a dummy value
        Ok(self.gen_number_const(0.0).into())
    }

    fn visit_toplevel(&mut self, top_level: &TopLevel) -> IRGenResult<BasicValueEnum<'ctx>> {
        match top_level {
            TopLevel::Function(func_def) => {
                // Save current function context
                let saved_function = self.current_function;
                let saved_block = self.builder.get_insert_block();

                self.visit_func_def(func_def)?;

                // Restore main function context
                if let Some(main_fn) = saved_function {
                    self.current_function = Some(main_fn);
                    if let Some(block) = saved_block {
                        self.builder.position_at_end(block);
                    }
                }

                Ok(self.gen_number_const(0.0).into())
            }
            TopLevel::Statement(stmt) => self.visit_stmt(stmt),
        }
    }

    fn visit_func_def(&mut self, func_def: &FuncDef) -> IRGenResult<BasicValueEnum<'ctx>> {
        let func_name = &func_def.name;

        // Create function signature with parameters
        let param_types: Vec<inkwell::types::BasicMetadataTypeEnum<'ctx>> = func_def
            .func
            .args
            .iter()
            .map(|_| self.type_mapping.get_number_type().into())
            .collect();

        let return_type = self.type_mapping.get_number_type();
        let fn_type = return_type.fn_type(&param_types, false);

        // Create function
        let function = self.module.add_function(func_name, fn_type, None);
        self.functions.insert(func_name.clone(), function);

        // Save current state
        let saved_variables = self.variables.clone();
        let saved_variable_types = self.variable_types.clone();
        let saved_function = self.current_function;

        // Enter function context
        self.enter_function(function);

        // Declare parameters as local variables
        for (i, param_name) in func_def.func.args.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let alloca =
                self.declare_variable(param_name, self.type_mapping.get_number_type().into())?;
            self.builder.build_store(alloca, param_value).map_err(|e| {
                IRGenError::InvalidOperation(format!(
                    "Failed to store parameter '{}': {}",
                    param_name, e
                ))
            })?;
        }

        // Generate function body
        let mut last_value = self.gen_number_const(0.0).into();
        for stmt in &func_def.func.body {
            // Check if current block already has a terminator
            if let Some(current_block) = self.builder.get_insert_block() {
                if current_block.get_terminator().is_some() {
                    // Current block is terminated, skip remaining statements
                    break;
                }
            }
            last_value = self.visit_stmt(stmt)?;
        }

        // Add return if not present
        if function
            .get_last_basic_block()
            .map_or(true, |bb| bb.get_terminator().is_none())
        {
            let ret_val = self.convert_to_return_type(last_value)?;
            self.builder.build_return(Some(&ret_val)).map_err(|e| {
                IRGenError::InvalidOperation(format!("Failed to build return: {}", e))
            })?;
        }

        // Restore state
        self.variables = saved_variables;
        self.variable_types = saved_variable_types;
        self.current_function = saved_function;

        Ok(self.gen_number_const(0.0).into())
    }

    fn visit_func(&mut self, _func: &Func) -> IRGenResult<BasicValueEnum<'ctx>> {
        // This should not be called directly in the new structure
        Err(IRGenError::InvalidOperation(
            "visit_func should not be called directly".to_string(),
        ))
    }

    // Delegate to separate modules
    fn visit_stmt(&mut self, stmt: &Stmt) -> IRGenResult<BasicValueEnum<'ctx>> {
        self.visit_stmt_impl(stmt)
    }

    fn visit_expr(&mut self, expr: &Expr) -> IRGenResult<BasicValueEnum<'ctx>> {
        self.visit_expr_impl(expr)
    }
}
