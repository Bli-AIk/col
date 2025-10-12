mod expr;

use crate::token::*;
use chumsky::{input::ValueInput, prelude::*};
use expr::*;
use std::collections::HashMap;

/*
program        → statement* EOF ;

statement      → exprStmt
               | varStmt ;

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
primary & atom → number | string | "true" | "false" | "null"
               | identifier
               | "(" expression ")" ;
*/

/// The top-level parser for a program, parsing a collection of function definitions.
pub(crate) fn funcs_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, HashMap<String, Func>, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    // Parser for a function's argument list, e.g., (a, b, c)
    let args = select! { Token::Identifier(s) => s.to_string() }
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::LeftParen), just(Token::RightParen));

    // A statement terminator is one or more semicolons or newlines
    let terminator = choice((just(Token::Semicolon), just(Token::Newline)))
        .repeated()
        .at_least(1);

    // Parser for a single function definition
    let func = just(Token::Function)
        .ignore_then(
            select! { Token::Identifier(s) => s.to_string() }.map_with(|name, e| (name, e.span())),
        )
        .then(args)
        .then(
            // The function body is a list of expressions separated by terminators
            expr_parser()
                .separated_by(terminator)
                // Allow empty lines at the start
                .allow_leading()
                // Allow empty lines at the end
                .allow_trailing()
                .collect()
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace)),
        )
        .map(|(((name, span), args), body)| ((name, span), Func { args, body }));

    // Parse multiple function definitions and collect them into a HashMap
    func.repeated()
        .collect::<Vec<_>>()
        .then_ignore(end())
        .validate(|fs, _span, emitter| {
            let mut funcs = HashMap::new();
            for ((name, name_span), f) in fs {
                if funcs.insert(name.clone(), f).is_some() {
                    emitter.emit(Rich::custom(
                        name_span,
                        format!("Function '{}' already exists", name),
                    ));
                }
            }
            funcs
        })
}

/// Parses a single expression, handling operator precedence, primitives, and function calls.
fn expr_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expr, extra::Err<Rich<'tokens, Token<'src>>>> + Clone
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    recursive(|expr| {
        let ident = select! { Token::Identifier(s) => s.to_string() };

        // region Primitives and atoms
        let atom = choice((
            select! { Token::Number(x) => Expr::Number(x.parse().unwrap()) },
            select! { Token::String(x) => Expr::String(x.to_string()) },
            just(Token::True).to(Expr::True(true)),
            just(Token::False).to(Expr::False(false)),
            just(Token::Null).to(Expr::Null),
            // Function call: identifier followed by a parenthesized list of expressions
            ident
                .clone()
                .then(
                    expr.clone()
                        .separated_by(just(Token::Comma))
                        .allow_trailing()
                        .collect()
                        .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
                )
                .map(|(name, args)| Expr::Call(name, args)),
            // A lone identifier is a variable
            ident.map(Expr::Identifier),
            // Parenthesized expression
            expr.clone()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .map(|e| Expr::Paren(Box::new(e))),
        ))
        .boxed();
        // endregion

        // region Unary operators
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
                atom, // Use atom here instead of the old 'primary'
            ))
        })
        .boxed();
        // endregion

        // region Multiplication, division, modulo
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

        // region Addition, subtraction
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

        // region Comparisons
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

        // region Equality
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

        // region Bitwise AND
        let bit_and = equality
            .clone()
            .foldl(
                just(Token::BitAnd)
                    .to(Expr::BitAnd as fn(_, _) -> _)
                    .then(equality)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Bitwise XOR
        let bit_xor = bit_and
            .clone()
            .foldl(
                just(Token::BitXor)
                    .to(Expr::BitXor as fn(_, _) -> _)
                    .then(bit_and)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Bitwise OR
        let bit_or = bit_xor
            .clone()
            .foldl(
                just(Token::BitOr)
                    .to(Expr::BitOr as fn(_, _) -> _)
                    .then(bit_xor)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Logical AND
        let logic_and = bit_or
            .clone()
            .foldl(
                just(Token::And)
                    .to(Expr::And as fn(_, _) -> _)
                    .then(bit_or)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Logical XOR
        let logic_xor = logic_and
            .clone()
            .foldl(
                just(Token::Xor)
                    .to(Expr::Xor as fn(_, _) -> _)
                    .then(logic_and)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Logical OR
        let logic_or = logic_xor
            .clone()
            .foldl(
                just(Token::Or)
                    .to(Expr::Or as fn(_, _) -> _)
                    .then(logic_xor)
                    .repeated(),
                |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
            )
            .boxed();
        // endregion

        // region Ternary operator
        let ternary = logic_or
            .clone()
            .then(
                just(Token::Question)
                    .ignore_then(expr.clone())
                    .then_ignore(just(Token::Colon))
                    .then(logic_or.clone()) // Ternary has specific precedence
                    .or_not(),
            )
            .map(|(cond, opt)| {
                if let Some((then_branch, else_branch)) = opt {
                    Expr::Ternary(Box::new(cond), Box::new(then_branch), Box::new(else_branch))
                } else {
                    cond
                }
            })
            .boxed();
        // endregion

        // region Assignment
        ternary
            .clone()
            .then(
                just(Token::Equal)
                    .to(Expr::Equal as fn(_, _) -> _)
                    .then(ternary)
                    .or_not(),
            )
            .map(|(lhs, opt)| {
                if let Some((op, rhs)) = opt {
                    op(Box::new(lhs), Box::new(rhs))
                } else {
                    lhs
                }
            })
            .boxed()

        // endregion
    })
}
