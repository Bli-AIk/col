use crate::codegen::ir_generator::IRGenerator;
use crate::codegen::jit::JITExecutor;
use crate::parser::program::Program;
use crate::parser::program_parser;
use crate::token::Token;
use chumsky::{input::Stream, prelude::*};
use inkwell::context::Context;
use logos::Logos;

/// Helper function to parse GML source code into an AST
pub(crate) fn parse_gml(src: &str) -> Program {
    let token_iter = Token::lexer(src).spanned().map(|(tok, span)| match tok {
        Ok(tok) => (tok, span.into()),
        Err(_) => (Token::Error, span.into()),
    });
    let stream = Stream::from_iter(token_iter).map((0..src.len()).into(), |(t, s): (_, _)| (t, s));
    match program_parser().parse(stream).into_result() {
        Ok(p) => p,
        Err(errs) => panic!("Parse failed: {:?}", errs),
    }
}

/// Helper function to compile and execute GML code, returning the main function result
pub(crate) fn compile_and_execute(src: &str) -> Result<f64, String> {
    let program = parse_gml(src);
    let context = Context::create();
    let mut ir_generator = IRGenerator::new(&context, "test_module");

    // Generate IR
    program
        .accept(&mut ir_generator)
        .map_err(|e| format!("IR generation failed: {:?}", e))?;

    // Verify module
    ir_generator
        .get_module()
        .verify()
        .map_err(|e| format!("Module verification failed: {}", e))?;

    // Execute with JIT
    let executor = JITExecutor::new(ir_generator.get_module())?;
    executor.execute_main()
}

/// Helper function to compile and execute a function by name
pub(crate) fn compile_and_execute_function(
    src: &str,
    func_name: &str,
    args: &[f64],
) -> Result<f64, String> {
    let program = parse_gml(src);
    let context = Context::create();
    let mut ir_generator = IRGenerator::new(&context, "test_module");

    program
        .accept(&mut ir_generator)
        .map_err(|e| format!("IR generation failed: {:?}", e))?;
    ir_generator
        .get_module()
        .verify()
        .map_err(|e| format!("Module verification failed: {}", e))?;

    let executor = JITExecutor::new(ir_generator.get_module())?;
    executor.execute_function(func_name, args)
}
