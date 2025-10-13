mod expr;

use crate::parser::expr::{Expr, Func, FuncDef, Program, Stmt, TopLevel};
use crate::token::*;
use chumsky::{input::ValueInput, prelude::*, recursive::Recursive};

/*
program        -> top_level* EOF ;

top_level      -> statement
               | function ;

function       -> "function" identifier "(" parameters? ")" block ;
parameters     -> identifier ( "," identifier )* ;


block          -> "{" statement* "}" ;

statement      -> exprStmt
               | varStmt
               | ifStmt
               | returnStmt
               | whileStmt
               | forStmt
               | block ;

exprStmt       -> [expression] terminator ;

varStmt        -> "var" variableDecl ("," variableDecl)* terminator;
variableDecl   -> IDENTIFIER ("=" expression)?;

ifStmt         -> "if" ("(" expression ")" | expression) "then"? statement ("else" statement)? ;


terminator     -> ( ";" | newline )+
---

expression     -> assignment ;

assignment     -> ternary ( "=" ternary )? ;

ternary        -> logic_or ( "?" expression ":" ternary )? ;

logic_or       -> logic_and ( "||" logic_and )* ;
logic_xor      -> logic_and ( "^^" logic_and )* ;
logic_and      -> bit_or ( "&&" bit_or )* ;
bit_or         -> bit_xor ( "|" bit_xor )* ;
bit_xor        -> bit_and ( "^" bit_and )* ;
bit_and        -> equality ( "&" equality )* ;
equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           -> factor ( ( "-" | "+" ) factor )* ;
factor         -> unary ( ( "/" | "*" | "%" ) unary )* ;
unary          -> ( "!" | "~" | "+" | "-" ) unary
               | primary ;
primary & atom -> number | string | "true" | "false" | "null"
               | identifier
               | "(" expression ")" ;
*/

/// The top-level parser for a program, parsing a collection of statements and function definitions.
pub(crate) fn program_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Program, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    // region Terminator
    let terminator = choice((just(Token::Semicolon), just(Token::Newline)))
        .repeated()
        .at_least(1)
        .ignored();
    // endregion

    // region Expressions and Statements (recursively defined)
    let mut statement_parser = Recursive::declare();
    let expr = expr_parser();

    let expr_stmt = expr
        .clone()
        .or_not()
        .then_ignore(terminator.clone())
        .map(|expr_opt| expr_opt.map(Stmt::Expr));

    let variable_decl = select! { Token::Identifier(s) => s.to_string() }
        .then(just(Token::Equal).ignore_then(expr.clone()).or_not());

    let var_stmt = just(Token::Var)
        .ignore_then(
            variable_decl
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .then_ignore(terminator.clone())
        .map(|vars| Some(Stmt::Var(vars)));

    let block_content = statement_parser
        .clone()
        .repeated()
        .collect::<Vec<Option<Stmt>>>()
        .map(|stmts| stmts.into_iter().flatten().collect::<Vec<Stmt>>());

    let block_stmt = block_content
        .clone()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
        .map(|stmts| Some(Stmt::Block(stmts)));

    let if_stmt = just(Token::If)
        .ignore_then(
            expr.clone()
                .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                .or(expr.clone()),
        )
        .then_ignore(just(Token::Then).or_not())
        .then(statement_parser.clone())
        .then_ignore(just(Token::Newline).repeated())
        .then(
            just(Token::Else)
                .ignore_then(statement_parser.clone())
                .or_not(),
        )
        .map(|((cond, then_opt), else_opt)| {
            // Ensure the 'then' branch is a block.
            let then_block = match then_opt.unwrap_or_else(|| Stmt::Block(Vec::new())) {
                Stmt::Block(stmts) => Stmt::Block(stmts),
                other_stmt => Stmt::Block(vec![other_stmt]),
            };

            // Ensure the 'else' branch, if it exists, is a block.
            let else_block = match else_opt {
                None => None,                                          // No else branch
                Some(None) => Some(Box::new(Stmt::Block(Vec::new()))), // else {}
                Some(Some(stmt)) => {
                    let else_stmt = match stmt {
                        Stmt::Block(stmts) => Stmt::Block(stmts),
                        other_stmt => Stmt::Block(vec![other_stmt]),
                    };
                    Some(Box::new(else_stmt))
                }
            };
            Some(Stmt::If(Box::new(cond), Box::new(then_block), else_block))
        });

    statement_parser.define(choice((
        expr_stmt.clone(),
        var_stmt.clone(),
        if_stmt,
        block_stmt,
    )));
    // endregion

    // region Function Body Block
    let func_body_block = statement_parser
        .clone()
        .repeated()
        .collect::<Vec<_>>()
        .map(|stmts| stmts.into_iter().flatten().collect()) // Filter out empty statements
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));
    // endregion

    // region Function Definition
    let parameters = select! { Token::Identifier(s) => s.to_string() }
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::LeftParen), just(Token::RightParen));

    let function = just(Token::Function)
        .ignore_then(select! { Token::Identifier(s) => s.to_string() })
        .then(parameters)
        .then(func_body_block)
        .map(|((name, args), body)| {
            TopLevel::Function(FuncDef {
                name,
                func: Func { args, body },
            })
        });
    // endregion

    // region Top Level Parser
    let top_level = choice((
        function.map(Some),
        statement_parser.map(|stmt_opt| stmt_opt.map(TopLevel::Statement)),
    ))
    .recover_with(skip_then_retry_until(any().ignored(), end()));

    let program = top_level
        .repeated()
        .collect::<Vec<_>>()
        .map(|top_levels| {
            let body = top_levels.into_iter().flatten().collect();
            Program { body }
        })
        .then_ignore(end());
    // endregion

    program
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
