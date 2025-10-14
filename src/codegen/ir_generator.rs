use crate::parser::visitor::Visitor;
use crate::parser::{expr::Expr, func::Func, func_def::FuncDef, program::Program, stmt::Stmt, top_level::TopLevel};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::*;
use inkwell::types::*;
use std::collections::HashMap;
use crate::codegen::TypeMapping;

/// Error types for IR generation
#[derive(Debug)]
pub enum IRGenError {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMismatch(String),
    InvalidOperation(String),
}

type IRGenResult<T> = Result<T, IRGenError>;

/// IR Generator that implements the Visitor pattern to generate LLVM IR
pub struct IRGenerator<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub type_mapping: TypeMapping<'ctx>,
    
    // Symbol tables
    variables: HashMap<String, PointerValue<'ctx>>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    
    // Current function context
    current_function: Option<FunctionValue<'ctx>>,
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
            functions: HashMap::new(),
            current_function: None,
        }
    }

    /// Generate IR for a constant number value
    fn gen_number_const(&self, value: f64) -> FloatValue<'ctx> {
        self.type_mapping.get_number_type().const_float(value)
    }

    /// Generate IR for a constant boolean value
    fn gen_bool_const(&self, value: bool) -> IntValue<'ctx> {
        self.type_mapping.get_bool_type().const_int(if value { 1 } else { 0 }, false)
    }

    /// Generate IR for a string constant
    fn gen_string_const(&self, value: &str) -> PointerValue<'ctx> {
        self.builder.build_global_string_ptr(value, "str_const")
            .expect("Failed to build string constant")
            .as_pointer_value()
    }

    /// Generate IR for a null value
    fn gen_null_const(&self) -> PointerValue<'ctx> {
        self.type_mapping.get_string_type().const_null()
    }

    /// Declare a variable in the current scope
    fn declare_variable(&mut self, name: &str, value_type: BasicTypeEnum<'ctx>) -> IRGenResult<PointerValue<'ctx>> {
        let alloca = self.builder.build_alloca(value_type, name)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to allocate variable '{}': {}", name, e)))?;
        
        self.variables.insert(name.to_string(), alloca);
        Ok(alloca)
    }

    /// Get a variable from the current scope
    fn get_variable(&self, name: &str) -> IRGenResult<PointerValue<'ctx>> {
        self.variables.get(name)
            .copied()
            .ok_or_else(|| IRGenError::UndefinedVariable(name.to_string()))
    }

    /// Load a variable's value
    fn load_variable(&self, name: &str) -> IRGenResult<BasicValueEnum<'ctx>> {
        let var_ptr = self.get_variable(name)?;
        // For LLVM 15+, we need to specify the type explicitly
        let load_type = self.type_mapping.get_number_type(); // Default to f64 for now
        self.builder.build_load(load_type, var_ptr, name)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to load variable '{}': {}", name, e)))
    }

    /// Store a value to a variable
    fn store_variable(&self, name: &str, value: BasicValueEnum<'ctx>) -> IRGenResult<()> {
        let var_ptr = self.get_variable(name)?;
        self.builder.build_store(var_ptr, value)
            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to store to variable '{}': {}", name, e)))?;
        Ok(())
    }

    /// Generate a function declaration
    fn declare_function(&mut self, name: &str, param_types: &[BasicMetadataTypeEnum<'ctx>], return_type: Option<BasicTypeEnum<'ctx>>) -> FunctionValue<'ctx> {
        let fn_type = self.type_mapping.get_function_type(return_type, param_types);
        let function = self.module.add_function(name, fn_type, None);
        self.functions.insert(name.to_string(), function);
        function
    }

    /// Get a function by name
    fn get_function(&self, name: &str) -> IRGenResult<FunctionValue<'ctx>> {
        self.functions.get(name)
            .copied()
            .ok_or_else(|| IRGenError::UndefinedFunction(name.to_string()))
    }

    /// Enter a new function context
    fn enter_function(&mut self, function: FunctionValue<'ctx>) {
        self.current_function = Some(function);
        // Create entry block
        let entry_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry_block);
    }

    /// Exit current function context
    fn exit_function(&mut self) {
        self.current_function = None;
        self.variables.clear(); // Clear local variables
    }

    /// Generate IR for binary operations
    fn gen_binary_op(&self, op: BinaryOp, lhs: BasicValueEnum<'ctx>, rhs: BasicValueEnum<'ctx>) -> IRGenResult<BasicValueEnum<'ctx>> {
        match (lhs, rhs) {
            (BasicValueEnum::FloatValue(l), BasicValueEnum::FloatValue(r)) => {
                let result = match op {
                    BinaryOp::Add => self.builder.build_float_add(l, r, "fadd").map(|v| v.into()),
                    BinaryOp::Sub => self.builder.build_float_sub(l, r, "fsub").map(|v| v.into()),
                    BinaryOp::Mul => self.builder.build_float_mul(l, r, "fmul").map(|v| v.into()),
                    BinaryOp::Div => self.builder.build_float_div(l, r, "fdiv").map(|v| v.into()),
                    BinaryOp::Eq => self.builder.build_float_compare(inkwell::FloatPredicate::OEQ, l, r, "feq").map(|v| v.into()),
                    BinaryOp::Ne => self.builder.build_float_compare(inkwell::FloatPredicate::ONE, l, r, "fne").map(|v| v.into()),
                    BinaryOp::Lt => self.builder.build_float_compare(inkwell::FloatPredicate::OLT, l, r, "flt").map(|v| v.into()),
                    BinaryOp::Le => self.builder.build_float_compare(inkwell::FloatPredicate::OLE, l, r, "fle").map(|v| v.into()),
                    BinaryOp::Gt => self.builder.build_float_compare(inkwell::FloatPredicate::OGT, l, r, "fgt").map(|v| v.into()),
                    BinaryOp::Ge => self.builder.build_float_compare(inkwell::FloatPredicate::OGE, l, r, "fge").map(|v| v.into()),
                    _ => return Err(IRGenError::InvalidOperation(format!("Unsupported float operation: {:?}", op))),
                };
                result.map_err(|e| IRGenError::InvalidOperation(format!("Float operation failed: {}", e)))
            },
            (BasicValueEnum::IntValue(l), BasicValueEnum::IntValue(r)) => {
                let result = match op {
                    BinaryOp::Add => self.builder.build_int_add(l, r, "iadd").map(|v| v.into()),
                    BinaryOp::Sub => self.builder.build_int_sub(l, r, "isub").map(|v| v.into()),
                    BinaryOp::Mul => self.builder.build_int_mul(l, r, "imul").map(|v| v.into()),
                    BinaryOp::Div => self.builder.build_int_signed_div(l, r, "idiv").map(|v| v.into()),
                    BinaryOp::Mod => self.builder.build_int_signed_rem(l, r, "irem").map(|v| v.into()),
                    BinaryOp::Eq => self.builder.build_int_compare(inkwell::IntPredicate::EQ, l, r, "ieq").map(|v| v.into()),
                    BinaryOp::Ne => self.builder.build_int_compare(inkwell::IntPredicate::NE, l, r, "ine").map(|v| v.into()),
                    BinaryOp::Lt => self.builder.build_int_compare(inkwell::IntPredicate::SLT, l, r, "ilt").map(|v| v.into()),
                    BinaryOp::Le => self.builder.build_int_compare(inkwell::IntPredicate::SLE, l, r, "ile").map(|v| v.into()),
                    BinaryOp::Gt => self.builder.build_int_compare(inkwell::IntPredicate::SGT, l, r, "igt").map(|v| v.into()),
                    BinaryOp::Ge => self.builder.build_int_compare(inkwell::IntPredicate::SGE, l, r, "ige").map(|v| v.into()),
                    BinaryOp::And => self.builder.build_and(l, r, "iand").map(|v| v.into()),
                    BinaryOp::Or => self.builder.build_or(l, r, "ior").map(|v| v.into()),
                    BinaryOp::Xor => self.builder.build_xor(l, r, "ixor").map(|v| v.into()),
                };
                result.map_err(|e| IRGenError::InvalidOperation(format!("Int operation failed: {}", e)))
            },
            _ => Err(IRGenError::TypeMismatch("Incompatible types for binary operation".to_string())),
        }
    }

    /// Get the LLVM module for output
    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
}

/// Binary operation types
#[derive(Debug, Clone, Copy)]
enum BinaryOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Ne, Lt, Le, Gt, Ge,
    And, Or, Xor,
}

impl<'ctx> Visitor<IRGenResult<BasicValueEnum<'ctx>>> for IRGenerator<'ctx> {
    fn visit_program(&mut self, program: &Program) -> IRGenResult<BasicValueEnum<'ctx>> {
        // Create a main function to hold global statements
        let main_fn_type = self.type_mapping.get_function_type(
            Some(self.type_mapping.get_number_type().into()),
            &[]
        );
        let main_function = self.module.add_function("main", main_fn_type, None);
        self.enter_function(main_function);
        
        let mut last_value = self.gen_number_const(0.0).into();
        for toplevel in &program.body {
            last_value = toplevel.accept(self)?;
        }
        
        // Only add return if the block doesn't have a terminator
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder.build_return(Some(&last_value))
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build return: {}", e)))?;
            }
        }
        
        self.exit_function();
        
        // Return a dummy value for program
        Ok(self.gen_number_const(0.0).into())
    }

    fn visit_toplevel(&mut self, toplevel: &TopLevel) -> IRGenResult<BasicValueEnum<'ctx>> {
        match toplevel {
            TopLevel::Statement(stmt) => self.visit_stmt(stmt),
            TopLevel::Function(func_def) => {
                // Save current function context
                let saved_function = self.current_function;
                let saved_block = self.builder.get_insert_block();
                
                // Generate function
                self.visit_func_def(func_def)?;
                
                // Restore main function context
                if let Some(main_fn) = saved_function {
                    self.current_function = Some(main_fn);
                    if let Some(block) = saved_block {
                        self.builder.position_at_end(block);
                    }
                }
                
                // Return a dummy value for function definitions
                Ok(self.gen_number_const(0.0).into())
            },
        }
    }

    fn visit_func_def(&mut self, func_def: &FuncDef) -> IRGenResult<BasicValueEnum<'ctx>> {
        // Create function signature
        let param_types: Vec<BasicMetadataTypeEnum<'ctx>> = func_def.func.args.iter()
            .map(|_| TypeMapping::parse_to_metadata_type(self.type_mapping.get_number_type().into()))
            .collect();
        
        // For now, assume all functions return number type
        let return_type = Some(self.type_mapping.get_number_type().into());
        
        let function = self.declare_function(&func_def.name, &param_types, return_type);
        
        // Save current state
        let saved_variables = self.variables.clone();
        let saved_function = self.current_function;
        
        // Generate function body
        self.enter_function(function);
        
        // Declare parameters as local variables
        for (i, param_name) in func_def.func.args.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let alloca = self.declare_variable(param_name, self.type_mapping.get_number_type().into())?;
            self.builder.build_store(alloca, param_value)
                .map_err(|e| IRGenError::InvalidOperation(format!("Failed to store parameter '{}': {}", param_name, e)))?;
        }
        
        // Generate function body statements
        let mut last_value = self.gen_number_const(0.0).into();
        for stmt in &func_def.func.body {
            last_value = self.visit_stmt(stmt)?;
        }
        
        // Build return instruction if current block doesn't have terminator
        if let Some(current_block) = self.builder.get_insert_block() {
            if current_block.get_terminator().is_none() {
                self.builder.build_return(Some(&last_value))
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build return: {}", e)))?;
            }
        }
        
        // Restore state
        self.variables = saved_variables;
        self.current_function = saved_function;
        
        Ok(function.as_global_value().as_pointer_value().into())
    }

    fn visit_func(&mut self, _func: &Func) -> IRGenResult<BasicValueEnum<'ctx>> {
        // This should not be called directly
        Err(IRGenError::InvalidOperation("visit_func should not be called directly".to_string()))
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> IRGenResult<BasicValueEnum<'ctx>> {
        match stmt {
            Stmt::Expr(expr) => self.visit_expr(expr),
            
            Stmt::Var(vars) => {
                let mut last_value = self.gen_number_const(0.0).into();
                for (name, init_expr) in vars {
                    let value = if let Some(expr) = init_expr {
                        self.visit_expr(expr)?
                    } else {
                        self.gen_number_const(0.0).into()
                    };
                    
                    let alloca = self.declare_variable(name, value.get_type())?;
                    self.builder.build_store(alloca, value)
                        .map_err(|e| IRGenError::InvalidOperation(format!("Failed to store variable '{}': {}", name, e)))?;
                    last_value = value;
                }
                Ok(last_value)
            },
            
            Stmt::If(cond, then_stmt, else_stmt) => {
                let cond_value = self.visit_expr(cond)?;
                
                let current_fn = self.current_function.ok_or_else(|| 
                    IRGenError::InvalidOperation("If statement outside function".to_string()))?;
                
                let then_block = self.context.append_basic_block(current_fn, "then");
                let else_block = self.context.append_basic_block(current_fn, "else");
                let merge_block = self.context.append_basic_block(current_fn, "merge");
                
                // Convert condition to i1 if needed
                let cond_i1 = match cond_value {
                    BasicValueEnum::IntValue(int_val) => {
                        if int_val.get_type() == self.type_mapping.get_bool_type() {
                            int_val
                        } else {
                            // Convert to bool by comparing with zero
                            self.builder.build_int_compare(
                                inkwell::IntPredicate::NE, 
                                int_val, 
                                int_val.get_type().const_zero(), 
                                "tobool"
                            ).map_err(|e| IRGenError::InvalidOperation(format!("Failed to convert to bool: {}", e)))?
                        }
                    },
                    BasicValueEnum::FloatValue(float_val) => {
                        self.builder.build_float_compare(
                            inkwell::FloatPredicate::ONE,
                            float_val,
                            float_val.get_type().const_zero(),
                            "tobool"
                        ).map_err(|e| IRGenError::InvalidOperation(format!("Failed to convert float to bool: {}", e)))?
                    },
                    _ => return Err(IRGenError::TypeMismatch("Invalid condition type".to_string())),
                };
                
                self.builder.build_conditional_branch(cond_i1, then_block, else_block)
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build conditional branch: {}", e)))?;
                
                // Generate then block
                self.builder.position_at_end(then_block);
                let then_value = self.visit_stmt(then_stmt)?;
                self.builder.build_unconditional_branch(merge_block)
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
                
                // Generate else block
                self.builder.position_at_end(else_block);
                let else_value = if let Some(else_stmt) = else_stmt {
                    self.visit_stmt(else_stmt)?
                } else {
                    self.gen_number_const(0.0).into()
                };
                self.builder.build_unconditional_branch(merge_block)
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build branch: {}", e)))?;
                
                // Merge block
                self.builder.position_at_end(merge_block);
                
                // Create phi node if values are compatible
                if then_value.get_type() == else_value.get_type() {
                    let phi = self.builder.build_phi(then_value.get_type(), "ifphi")
                        .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build phi: {}", e)))?;
                    phi.add_incoming(&[(&then_value, then_block), (&else_value, else_block)]);
                    Ok(phi.as_basic_value())
                } else {
                    Ok(then_value)
                }
            },
            
            Stmt::Block(stmts) => {
                let mut last_value = self.gen_number_const(0.0).into();
                for stmt in stmts {
                    last_value = self.visit_stmt(stmt)?;
                }
                Ok(last_value)
            },
            
            Stmt::Return(expr_opt) => {
                let value = if let Some(expr) = expr_opt {
                    self.visit_expr(expr)?
                } else {
                    self.gen_number_const(0.0).into()
                };
                self.builder.build_return(Some(&value))
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build return: {}", e)))?;
                Ok(value)
            },
            
            // TODO: Implement other statement types
            _ => Ok(self.gen_number_const(0.0).into()),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> IRGenResult<BasicValueEnum<'ctx>> {
        match expr {
            Expr::Number(n) => Ok(self.gen_number_const(*n).into()),
            Expr::String(s) => Ok(self.gen_string_const(s).into()),
            Expr::True(_) => Ok(self.gen_bool_const(true).into()),
            Expr::False(_) => Ok(self.gen_bool_const(false).into()),
            Expr::Null => Ok(self.gen_null_const().into()),
            
            Expr::Identifier(name) => self.load_variable(name),
            
            Expr::Call(name, args) => {
                let function = self.get_function(name)?;
                let arg_values: Result<Vec<_>, _> = args.iter()
                    .map(|arg| self.visit_expr(arg))
                    .collect();
                let arg_values = arg_values?;
                
                // Convert BasicValueEnum to BasicMetadataValueEnum
                let metadata_args: Vec<BasicMetadataValueEnum> = arg_values.iter()
                    .map(|val| match val {
                        BasicValueEnum::IntValue(v) => (*v).into(),
                        BasicValueEnum::FloatValue(v) => (*v).into(),
                        BasicValueEnum::PointerValue(v) => (*v).into(),
                        BasicValueEnum::ArrayValue(v) => (*v).into(),
                        BasicValueEnum::StructValue(v) => (*v).into(),
                        BasicValueEnum::VectorValue(v) => (*v).into(),
                        BasicValueEnum::ScalableVectorValue(v) => (*v).into(),
                    })
                    .collect();
                
                let call_value = self.builder.build_call(function, &metadata_args, "call")
                    .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build call: {}", e)))?;
                
                call_value.try_as_basic_value().left()
                    .ok_or_else(|| IRGenError::InvalidOperation("Function call returned void".to_string()))
            },
            
            // Binary operations
            Expr::Addition(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Add, l, r)
            },
            Expr::Subtraction(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Sub, l, r)
            },
            Expr::Multiplication(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Mul, l, r)
            },
            Expr::Division(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Div, l, r)
            },
            Expr::Percent(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Mod, l, r)
            },
            
            // Comparison operations
            Expr::EqualEqual(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Eq, l, r)
            },
            Expr::NotEqual(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Ne, l, r)
            },
            Expr::Less(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Lt, l, r)
            },
            Expr::LessEqual(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Le, l, r)
            },
            Expr::Greater(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Gt, l, r)
            },
            Expr::GreaterEqual(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Ge, l, r)
            },
            
            // Logical operations
            Expr::And(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::And, l, r)
            },
            Expr::Or(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Or, l, r)
            },
            Expr::Xor(lhs, rhs) => {
                let l = self.visit_expr(lhs)?;
                let r = self.visit_expr(rhs)?;
                self.gen_binary_op(BinaryOp::Xor, l, r)
            },
            
            // Unary operations
            Expr::Not(expr) => {
                let value = self.visit_expr(expr)?;
                match value {
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_not(int_val, "not")
                            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build not: {}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(IRGenError::TypeMismatch("Not operation requires integer type".to_string())),
                }
            },
            Expr::Negative(expr) => {
                let value = self.visit_expr(expr)?;
                match value {
                    BasicValueEnum::FloatValue(float_val) => {
                        let result = self.builder.build_float_neg(float_val, "fneg")
                            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build float neg: {}", e)))?;
                        Ok(result.into())
                    },
                    BasicValueEnum::IntValue(int_val) => {
                        let result = self.builder.build_int_neg(int_val, "ineg")
                            .map_err(|e| IRGenError::InvalidOperation(format!("Failed to build int neg: {}", e)))?;
                        Ok(result.into())
                    },
                    _ => Err(IRGenError::TypeMismatch("Negation requires numeric type".to_string())),
                }
            },
            Expr::Positive(expr) => {
                // Positive is a no-op
                self.visit_expr(expr)
            },
            
            // Assignment operations
            Expr::Equal(lhs, rhs) => {
                let rhs_value = self.visit_expr(rhs)?;
                if let Expr::Identifier(name) = lhs.as_ref() {
                    self.store_variable(name, rhs_value)?;
                    Ok(rhs_value)
                } else {
                    Err(IRGenError::InvalidOperation("Can only assign to variables".to_string()))
                }
            },
            
            Expr::Paren(expr) => self.visit_expr(expr),
            
            // TODO: Implement remaining expression types
            _ => Ok(self.gen_number_const(0.0).into()),
        }
    }
}