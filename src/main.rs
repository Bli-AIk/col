use crate::token::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{input::Stream, prelude::*};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::*;
use std::fs;
use utils::colorize;

mod codegen;
mod parser;
mod token;
mod utils;

fn main() {
    let path = "Sample.gml";

    // Read source file
    let content = match read_source_file(path) {
        Ok(content) => content,
        Err(_) => return,
    };

    // Display original code
    display_original_code(&content);

    // Perform lexical analysis
    perform_lexical_analysis(&content);

    // Parse the source code
    let program = match parse_source_code(&content) {
        Ok(program) => program,
        Err(_) => return,
    };

    // Build symbol table
    build_and_display_symbol_table(&program);

    // Generate LLVM IR and execute with JIT
    generate_ir_and_execute(&program);
}

/// Read and validate the source file
fn read_source_file(path: &str) -> Result<String, ()> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => {
            eprintln!();
            eprintln!(
                "{} {}",
                format!("Failed to read '{}':", path).bright_red(),
                e
            );
            std::process::exit(1);
        }
    }
}

/// Display the original source code
fn display_original_code(content: &str) {
    println!();
    println!("----------Output----------");
    println!();
    println!("{}\n {}\n", "Original Code:".green(), content);
}

/// Perform lexical analysis and display tokens
fn perform_lexical_analysis(content: &str) {
    lex_with_output(content);
}

/// Parse source code and return AST
fn parse_source_code(content: &str) -> Result<program::Program, ()> {
    let token_iter = Token::lexer(content)
        .spanned()
        .map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(_) => {
                println!("Error token encountered: {:?}", &content[span.clone()]);
                (Token::Error, span.into())
            }
        });

    let token_stream =
        Stream::from_iter(token_iter).map((0..content.len()).into(), |(t, s): (_, _)| (t, s));

    println!();
    match program_parser().parse(token_stream).into_result() {
        Ok(program) => {
            display_ast(&program);
            Ok(program)
        }
        Err(errs) => {
            display_parse_errors(errs, content);
            Err(())
        }
    }
}

/// Display the parsed AST
fn display_ast(program: &program::Program) {
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
        colorize::colorize_brackets(&debug_str)
    );
}

/// Display parsing errors
fn display_parse_errors(errors: Vec<Rich<Token>>, content: &str) {
    for err in errors {
        Report::build(ReportKind::Error, ((), err.span().into_range()))
            .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
            .with_message(err.to_string())
            .with_label(
                Label::new(((), err.span().into_range()))
                    .with_message(err.reason().to_string())
                    .with_color(Color::Red),
            )
            .finish()
            .eprint(Source::from(content))
            .unwrap();
    }
}

/// Build symbol table and display it
fn build_and_display_symbol_table(program: &program::Program) {
    let is_pretty_print_symbol_table = true;
    let mut root_scope = visitor::symbol_table_builder::Scope::new();
    let mut builder = visitor::symbol_table_builder::SymbolTableBuilder::new(&mut root_scope);
    program.accept(&mut builder);

    let symbol_table_debug_str = if is_pretty_print_symbol_table {
        format!("{:#?}", root_scope)
    } else {
        format!("{:?}", root_scope)
    };

    println!(
        "{}\n {}\n",
        "Symbol Table:".green(),
        colorize::colorize_brackets(&symbol_table_debug_str)
    );
}

/// Generate LLVM IR and execute with JIT
fn generate_ir_and_execute(program: &program::Program) {
    println!("{}", "Generating LLVM IR...".green());
    let context = inkwell::context::Context::create();
    let mut ir_generator = codegen::ir_generator::IRGenerator::new(&context, "main_module");

    match program.accept(&mut ir_generator) {
        Ok(_) => {
            println!("{}", "IR Generation completed successfully!".green());

            // Display and save generated IR
            display_and_save_ir(&ir_generator);

            // Verify and execute the module
            verify_and_execute_module(&ir_generator);
        }
        Err(e) => {
            println!("{}", format!("IR Generation failed: {:?}", e).red());
        }
    }
}

/// Display the generated LLVM IR and save to file
fn display_and_save_ir(ir_generator: &codegen::ir_generator::IRGenerator) {
    // Display generated IR
    println!("\n{}", "Generated LLVM IR:".green());
    let ir_string = ir_generator.get_module().print_to_string().to_string();
    println!("{}", ir_string);

    // Save IR to file
    save_ir_to_file(&ir_string);
}

/// Save LLVM IR to file
fn save_ir_to_file(ir_string: &str) {
    let ir_path = "Sample.ll";
    match fs::write(ir_path, ir_string) {
        Ok(_) => println!("{} '{}'", "LLVM IR saved to".green(), ir_path),
        Err(e) => eprintln!("{} {}", "Failed to write IR file:".red(), e),
    }
}

/// Verify the module and execute with JIT if successful
fn verify_and_execute_module(ir_generator: &codegen::ir_generator::IRGenerator) {
    if let Err(errors) = ir_generator.get_module().verify() {
        println!("{}", "Module verification failed:".red());
        println!("{}", errors.to_string().red());
    } else {
        println!("{}", "Module verification passed!".green());
        execute_with_jit(ir_generator);
    }
}

/// Execute the generated code using JIT
fn execute_with_jit(ir_generator: &codegen::ir_generator::IRGenerator) {
    println!("\n{}", "Executing with JIT...".green());

    match codegen::jit::JITExecutor::new(ir_generator.get_module()) {
        Ok(executor) => {
            // Execute main function
            execute_main_function(&executor);

            // Try to execute test functions
            execute_test_functions(&executor);
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
    // Try to execute the test_func if it exists
    match executor.execute_function("test_func", &[10.0]) {
        Ok(result) => {
            println!("{} {}", "test_func(10.0) returned:".green(), result);
        }
        Err(e) => {
            println!("{}", format!("test_func execution failed: {}", e).yellow());
        }
    }
}
