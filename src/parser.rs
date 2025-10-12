mod expr;

use expr::*;

use crate::token::*;
use chumsky::{
    // Stream 用于将词法分析器的输出转换为 chumsky 可以使用的输入流
    input::{Stream, ValueInput},
    // prelude::* 包含了常用的解析器构建器和类型
    prelude::*,
};


fn parser<'tokens, 'src: 'tokens, I>()
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
        unary
    })
}

