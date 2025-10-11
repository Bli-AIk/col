use crate::token::Token;
use logos::Logos;
use owo_colors::OwoColorize;
use std::fs;

mod parser;
mod token;

fn main() {
    let content = fs::read_to_string("Sample.gml").expect("Something went wrong reading the file");
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
                let x = "Error token encountered";
                println!("{} : {:?}", x.red(), token.red())
            }
        }
    }
}
