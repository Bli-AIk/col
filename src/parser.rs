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

expression     → assignment ;

assignment     → ternary ( "=" ternary )? ;

ternary        → logic_or ( "?" expression ":" ternary )? ;

logic_or       → logic_and ( "||" logic_and )* ;
logic_xor      → logic_and ( "^^" logic_and )* ;
logic_and      → bit_or ( "&&" bit_or )* ;
bit_or         → bit_xor ( "|" bit_xor )* ;
bit_xor        → bit_and ( "^" bit_and )* ;
bit_and        → equality ( "&" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" | "%" ) unary )* ;
unary          → ( "!" | "~" | "+" | "-" ) unary
               | primary ;
primary        → number | string | "true" | "false" | "null"
               | identifier
               | "(" expression ")" ;
*/

pub(crate) fn parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    let expression = recursive(|expr| {
        // region number
        let number = select! {
            Token::Number(x) => Expr::Number(x.parse().unwrap()),
        };
        // endregion

        // region string
        let string = select! {
            Token::String(x) => Expr::String(x.to_string()),
        };
        // endregion

        // region bool_true
        let bool_true = select! {
            Token::True  => Expr::True(true),
        };
        // endregion

        // region bool_false
        let bool_false = select! {
            Token::False  => Expr::False(false),
        };
        // endregion

        // region null
        let null = select! {
            Token::Null  => Expr::Null,
        };
        // endregion

        // region identifier
        let identifier = select! {
            Token::Identifier(x) => Expr::Identifier(x.to_string()),
        };
        // endregion

        // region primary
        let primary = choice((
            number,
            string,
            bool_true,
            bool_false,
            null,
            identifier,
            expr.clone()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .map(|e| Expr::Paren(Box::new(e))),
        ))
        .boxed();
        // endregion

        // region unary
        let unary = recursive(|unary| {
            choice((
                just(Token::Not)
                    .ignore_then(unary.clone())
                    .map(|e| Expr::Not(Box::new(e))),
                just(Token::BitNot)
                    .ignore_then(unary.clone())
                    .map(|e| Expr::BitNot(Box::new(e))),
                just(Token::Plus)
                    .ignore_then(unary.clone())
                    .map(|e| Expr::Positive(Box::new(e))),
                just(Token::Minus)
                    .ignore_then(unary.clone())
                    .map(|e| Expr::Negative(Box::new(e))),
                primary,
            ))
        })
        .boxed();

        // endregion

        // region factor
        let factor = unary
            .clone()
            .foldl(
                choice((
                    just(Token::Star).to(Expr::Multiplication as fn(_, _) -> _),
                    just(Token::Slash).to(Expr::Division as fn(_, _) -> _),
                    just(Token::Percent).to(Expr::Percent as fn(_, _) -> _),
                ))
                .then(unary)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region term
        let term = factor
            .clone()
            .foldl(
                choice((
                    just(Token::Plus).to(Expr::Addition as fn(_, _) -> _),
                    just(Token::Minus).to(Expr::Subtraction as fn(_, _) -> _),
                ))
                .then(factor)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region comparison
        let comparison = term
            .clone()
            .foldl(
                choice((
                    just(Token::Greater).to(Expr::Greater as fn(_, _) -> _),
                    just(Token::GreaterEqual).to(Expr::GreaterEqual as fn(_, _) -> _),
                    just(Token::Less).to(Expr::Less as fn(_, _) -> _),
                    just(Token::LessEqual).to(Expr::LessEqual as fn(_, _) -> _),
                ))
                .then(term)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region equality
        let equality = comparison
            .clone()
            .foldl(
                choice((
                    just(Token::EqualEqual).to(Expr::EqualEqual as fn(_, _) -> _),
                    just(Token::NotEqual).to(Expr::NotEqual as fn(_, _) -> _),
                ))
                .then(comparison)
                .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region bit_and
        let bit_and = equality
            .clone()
            .foldl(
                choice((just(Token::BitAnd).to(Expr::BitAnd as fn(_, _) -> _),))
                    .then(equality)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region bit_xor
        let bit_xor = bit_and
            .clone()
            .foldl(
                choice((just(Token::BitXor).to(Expr::BitXor as fn(_, _) -> _),))
                    .then(bit_and)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region bit_or
        let bit_or = bit_xor
            .clone()
            .foldl(
                choice((just(Token::BitOr).to(Expr::BitOr as fn(_, _) -> _),))
                    .then(bit_xor)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region logic_and
        let logic_and = bit_or
            .clone()
            .foldl(
                choice((just(Token::And).to(Expr::And as fn(_, _) -> _),))
                    .then(bit_or)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region logic_xor
        let logic_xor = logic_and
            .clone()
            .foldl(
                choice((just(Token::Xor).to(Expr::Xor as fn(_, _) -> _),))
                    .then(logic_and)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region logic_or
        let logic_or = logic_xor
            .clone()
            .foldl(
                choice((just(Token::Or).to(Expr::Or as fn(_, _) -> _),))
                    .then(logic_xor)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region ternary
        let ternary = recursive(|ternary| {
            logic_or
                .clone()
                .then(
                    just(Token::Question)
                        .ignore_then(expr.clone())
                        .then_ignore(just(Token::Colon))
                        .then(ternary.clone())
                        .or_not(),
                )
                .map(|(cond, opt)| {
                    if let Some((then_branch, else_branch)) = opt {
                        Expr::Ternary(Box::new(cond), Box::new(then_branch), Box::new(else_branch))
                    } else {
                        cond
                    }
                })
        })
        .boxed();
        // endregion

        // region assignment
        let assignment = ternary
            .clone()
            .foldl(
                choice((just(Token::Equal).to(Expr::Equal as fn(_, _) -> _),))
                    .then(ternary)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        let result = assignment;
        // Ignore NewLine for now
        result //.padded_by(just(Token::Newline).repeated()).boxed()
    });

    // region expr_stmt
    let expr_stmt = expression
        .clone()
        .then(choice((just(Token::Semicolon), just(Token::Newline))).repeated())
        .map(|(e, _)| e)
        .boxed();
    // endregion

    expr_stmt
}
