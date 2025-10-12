mod expr;

use expr::*;

use crate::token::*;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};


pub(crate) fn parser<'tokens, 'src: 'tokens, I>()
    -> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token=Token<'src>, Span=SimpleSpan>,
{
    recursive(|expr| {
        let number = select! {
            Token::Number(x) => Expr::Number(x.parse().unwrap()),
        };

        let atom = number;
        let unary = choice((
            just(Token::Plus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Pos(Box::new(e))),
            just(Token::Minus)
                .ignore_then(expr.clone())
                .map(|e| Expr::Neg(Box::new(e))),
            atom.clone()
        ));



        let result = unary;
        // Ignore NewLine for now
        result.padded_by(just(Token::Newline).repeated())
    })
}

