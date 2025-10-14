use crate::token::*;
use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::{input::Stream, prelude::*};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::*;
use std::fs;
use utils::colorize;

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
