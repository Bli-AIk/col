mod expr;

use expr::*;

use crate::token::*;
use chumsky::{
    input::ValueInput,
    prelude::*,
};

pub(crate) fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let number = select! {
            Token::Number(x) => Expr::Num(x.parse().unwrap()),
        };

        let atom = choice((
            number.clone(),
            expr.clone()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .map(|e| Expr::Paren(Box::new(e))),
        ));

        let unary = choice((
            just(Token::Plus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Pos(Box::new(e))),
            just(Token::Minus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Neg(Box::new(e))),
            atom,
        ));

        let product = unary.clone().foldl(
            choice((
                just(Token::Star).to(Expr::Mul as fn(_, _) -> _),
                just(Token::Slash).to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let sum = product.clone().foldl(
            choice((
                just(Token::Plus).to(Expr::Add as fn(_, _) -> _),
                just(Token::Minus).to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        let result = sum;
        // Ignore NewLine for now
        result.padded_by(just(Token::Newline).repeated())
    })
}
