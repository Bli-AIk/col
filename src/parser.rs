mod expr;

use expr::*;

use crate::token::*;
use chumsky::{input::ValueInput, prelude::*};

/*
program        → statement* EOF ;

statement      → exprStmt
               | xxxStmt ;

exprStmt       → expression (";" | newline);

---

expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "+" | "-" ) unary
               | primary ;
primary        → number | string | "true" | "false" | "null"
               | "(" expression ")" ;
*/

pub(crate) fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let number = select! {
            Token::Number(x) => Expr::Number(x.parse().unwrap()),
        };

        let string = select! {
            Token::String(x) => Expr::String(x.to_string()),
        };

        let bool_true = select! {
            Token::True  => Expr::True(true),
        };
        let bool_false = select! {
            Token::False  => Expr::False(false),
        };

        let primary = choice((
            number,
            string,
            bool_true,
            bool_false,
            expr.clone()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .map(|e| Expr::Paren(Box::new(e))),
        ));

        let unary = choice((
            just(Token::Not)
                .ignore_then(expr.clone())
                .map(|e| Expr::Not(Box::new(e))),
            just(Token::Plus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Positive(Box::new(e))),
            just(Token::Minus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Negative(Box::new(e))),
            primary,
        ));

        let factor = unary.clone().foldl(
            choice((
                just(Token::Star).to(Expr::Multiplication as fn(_, _) -> _),
                just(Token::Slash).to(Expr::Division as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let term = factor.clone().foldl(
            choice((
                just(Token::Plus).to(Expr::Addition as fn(_, _) -> _),
                just(Token::Minus).to(Expr::Subtraction as fn(_, _) -> _),
            ))
            .then(factor)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let result = term;
        // Ignore NewLine for now
        result.padded_by(just(Token::Newline).repeated())
    })
}
