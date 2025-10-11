use crate::token::Token;
use logos::Logos;

mod token;

fn main() {
    let mut lex = token::Token::lexer(
        r#"var y;







    var x = 123 + 456;
    x == 123


    q = "why"

    air=1230;

    repeat (5){
        x++
    }
    "#,
    );

    while let Some(result) = lex.next() {
        match result {
            Ok(token) => {
                if (token != Token::Newline) {
                    print!("{:?} ", token);
                } else {
                    print!("//{:?} ", token);
                    println!();
                }
            }
            Err(_) => println!("Error token encountered"),
        }
    }
}
