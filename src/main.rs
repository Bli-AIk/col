use crate::token::*;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};
use logos::Logos;
use owo_colors::OwoColorize;
use parser::*;
use std::fs;

mod parser;
mod token;

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
    match parser().parse(token_stream).into_result() {
        Ok(expr) => println!("{} {:?}", "Parsed:".green(), expr),
        Err(errs) => println!("{} {:?}", "Parse errors:".red(), errs.red()),
    }
}
