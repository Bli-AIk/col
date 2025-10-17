use crate::codegen;
use crate::parser::*;
use owo_colors::OwoColorize;

/// Handle code generation and execution
pub struct CodeGenHandler;

impl CodeGenHandler {
    /// Generate LLVM IR and execute with JIT
    pub fn generate_ir_and_execute(program: &program::Program) {
        println!("{}", "Generating LLVM IR...".green());
        let context = inkwell::context::Context::create();
        let mut ir_generator = codegen::ir_generator::IRGenerator::new(&context, "main_module");

        match program.accept(&mut ir_generator) {
            Ok(_) => {
                println!("{}", "IR Generation completed successfully!".green());

                // Display and save generated IR
                crate::handler::output_handler::OutputHandler::display_and_save_ir(&ir_generator);

                // Verify and execute the module
                Self::verify_and_execute_module(&ir_generator);
            }
            Err(e) => {
                println!("{}", format!("IR Generation failed: {:?}", e).red());
            }
        }
    }

    /// Verify the module and execute with JIT if successful
    fn verify_and_execute_module(ir_generator: &codegen::ir_generator::IRGenerator) {
        if let Err(errors) = ir_generator.get_module().verify() {
            println!("{}", "Module verification failed:".red());
            println!("{}", errors.to_string().red());
        } else {
            println!("{}", "Module verification passed!".green());
            Self::execute_with_jit(ir_generator);
        }
    }

    /// Execute the generated code using JIT
    fn execute_with_jit(ir_generator: &codegen::ir_generator::IRGenerator) {
        println!("\n{}", "Executing with JIT...".green());

        match codegen::jit::JITExecutor::new(ir_generator.get_module()) {
            Ok(executor) => {
                // Execute main function
                Self::execute_main_function(&executor);

                // Try to execute test functions
                Self::execute_test_functions(&executor);
            }
            Err(e) => {
                println!("{}", format!("Failed to create JIT executor: {}", e).red());
            }
        }
    }

    /// Execute the main function
    fn execute_main_function(executor: &codegen::jit::JITExecutor) {
        match executor.execute_main() {
            Ok(result) => {
                println!("{} {}", "Main function returned:".green(), result);
            }
            Err(e) => {
                println!("{}", format!("JIT execution failed: {}", e).red());
            }
        }
    }

    /// Execute test functions if they exist
    fn execute_test_functions(executor: &codegen::jit::JITExecutor) {
        // Try to execute the test_short_circuit function if it exists
        match executor.execute_function("test_short_circuit", &[]) {
            Ok(result) => {
                println!("{} {}", "test_short_circuit() returned:".green(), result);
            }
            Err(e) => {
                println!(
                    "{}",
                    format!("test_short_circuit execution failed: {}", e).yellow()
                );
            }
        }

        // Try to execute the test_loops function if it exists
        match executor.execute_function("test_loops", &[]) {
            Ok(result) => {
                println!("{} {}", "test_loops() returned:".green(), result);
            }
            Err(e) => {
                println!("{}", format!("test_loops execution failed: {}", e).yellow());
            }
        }
    }
}
