use logos::Logos;

mod token;

fn main() {
    let mut lex = crate::token::Token::lexer(r#"var x = 123 + 456;"#);

    while let Some(result) = lex.next() {
        match result {
            Ok(token) => println!("{:?}", token),
            Err(_) => println!("Error token encountered"),
        }
    }
}
