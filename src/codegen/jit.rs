use inkwell::OptimizationLevel;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;

pub struct JITExecutor<'ctx> {
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> JITExecutor<'ctx> {
    pub fn new(module: &Module<'ctx>) -> Result<Self, String> {
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|e| format!("Failed to create JIT execution engine: {}", e))?;

        Ok(Self { execution_engine })
    }

    /// Execute the main function and return its result
    pub fn execute_main(&self) -> Result<f64, String> {
        unsafe {
            let main_fn: JitFunction<unsafe extern "C" fn() -> f64> = self
                .execution_engine
                .get_function("main")
                .map_err(|e| format!("Failed to get main function: {}", e))?;

            Ok(main_fn.call())
        }
    }

    /// Execute a function by name with given arguments
    pub fn execute_function(&self, name: &str, args: &[f64]) -> Result<f64, String> {
        match args.len() {
            0 => unsafe {
                let func: JitFunction<unsafe extern "C" fn() -> f64> = self
                    .execution_engine
                    .get_function(name)
                    .map_err(|e| format!("Failed to get function '{}': {}", name, e))?;

                Ok(func.call())
            },
            1 => unsafe {
                let func: JitFunction<unsafe extern "C" fn(f64) -> f64> = self
                    .execution_engine
                    .get_function(name)
                    .map_err(|e| format!("Failed to get function '{}': {}", name, e))?;

                Ok(func.call(args[0]))
            },
            2 => unsafe {
                let func: JitFunction<unsafe extern "C" fn(f64, f64) -> f64> = self
                    .execution_engine
                    .get_function(name)
                    .map_err(|e| format!("Failed to get function '{}': {}", name, e))?;

                Ok(func.call(args[0], args[1]))
            },
            _ => Err(format!("Unsupported number of arguments: {}", args.len())),
        }
    }

    /// Get the execution engine reference for advanced usage
    pub fn get_execution_engine(&self) -> &ExecutionEngine<'ctx> {
        &self.execution_engine
    }
}
