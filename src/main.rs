use crate::token::Token;
use logos::Logos;
use owo_colors::OwoColorize;
use std::fs;

mod parser;
mod token;

fn main() {
    let path = "Sample.gml";
    let content = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", format!("Failed to read '{}':", path).bright_red(), e);
            std::process::exit(1);
        }
    };
    let mut lex = Token::lexer(&content);

    while let Some(result) = lex.next() {
        match result {
            Ok(token) => {
                if token != Token::Newline {
                    print!("{:?} ", token);
                } else {
                    print!("{:?} ", token.green());
                    println!();
                }
            }
            Err(token) => {
                let x = "Error token encountered :";
                println!("{} {:?}", x.red(), token.red())
            }
        }
    }
}
