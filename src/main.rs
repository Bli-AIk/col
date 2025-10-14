use crate::token::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{input::Stream, prelude::*};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::*;
use std::fs;
use codegen::{ir_generator, jit};
use utils::colorize;

mod codegen;
mod parser;
mod token;
mod utils;

fn main() {
    let path = "Sample.gml";
    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!();
            eprintln!(
                "{} {}",
                format!("Failed to read '{}':", path).bright_red(),
                e
            );
            std::process::exit(1);
        }
    };

    println!();
    println!("----------Output----------");
    println!();
    println!(
        "{}\n {}\n",
        "Original Code:".green(),
        &content
    );

    lex_with_output(&content);

    let token_iter = Token::lexer(&content)
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

            let is_pretty_print_symbol_table = true;
            let mut root_scope = visitor::symbol_table_builder::Scope::new();
            let mut builder =
                visitor::symbol_table_builder::SymbolTableBuilder::new(&mut root_scope);
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

            // Generate LLVM IR
            println!("{}", "Generating LLVM IR...");
            let context = inkwell::context::Context::create();
            let mut ir_generator = ir_generator::IRGenerator::new(&context, "main_module");
            
            match program.accept(&mut ir_generator) {
                Ok(_) => {
                    println!("{}", "IR Generation completed successfully!".green());
                    
                    // Print the generated LLVM IR
                    println!("\n{}", "Generated LLVM IR:".green());
                    println!("{}", ir_generator.get_module().print_to_string().to_string());

                    // Save IR to a file
                    let ir_string = ir_generator.get_module().print_to_string().to_string();
                    let ir_path = "Sample.ll";

                    match fs::write(ir_path, &ir_string) {
                        Ok(_) => println!("{} '{}'", "LLVM IR saved to".green(), ir_path),
                        Err(e) => eprintln!("{} {}", "Failed to write IR file:".red(), e),
                    }


                    // Verify the module
                    if let Err(errors) = ir_generator.get_module().verify() {
                        println!("{}", "Module verification failed:".red());
                        println!("{}", errors.to_string().red());
                    } else {
                        println!("{}", "Module verification passed!".green());
                        
                        // Try JIT execution
                        println!("\n{}", "Executing with JIT...");
                        match jit::JITExecutor::new(ir_generator.get_module()) {
                            Ok(executor) => {
                                match executor.execute_main() {
                                    Ok(result) => {
                                        println!("{} {}", "Main function returned:".green(), result);
                                    },
                                    Err(e) => {
                                        println!("{}", format!("JIT execution failed: {}", e).red());
                                    }
                                }
                                
                                // Try to execute the test_func if it exists
                                match executor.execute_function("test_func", &[10.0]) {
                                    Ok(result) => {
                                        println!("{} {}", "test_func(10.0) returned:".green(), result);
                                    },
                                    Err(e) => {
                                        println!("{}", format!("test_func execution failed: {}", e).yellow());
                                    }
                                }
                            },
                            Err(e) => {
                                println!("{}", format!("Failed to create JIT executor: {}", e).red());
                            }
                        }
                    }
                },
                Err(e) => {
                    println!("{}", format!("IR Generation failed: {:?}", e).red());
                }
            }
        }
        Err(errs) => {
            for err in errs {
                Report::build(ReportKind::Error, ((), err.span().into_range()))
                    .with_config(ariadne::Config::new().with_index_type(ariadne::IndexType::Byte))
                    //.with_code(1)
                    .with_message(err.to_string())
                    .with_label(
                        Label::new(((), err.span().into_range()))
                            .with_message(err.reason().to_string())
                            .with_color(Color::Red),
                    )
                    .finish()
                    .eprint(Source::from(&content))
                    .unwrap();
            }
        }
    }
}
