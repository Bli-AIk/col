mod expr;

use crate::parser::expr::{Expr, Func, FuncDef, Program, Stmt, TopLevel};
use crate::token::*;
use chumsky::{input::ValueInput, prelude::*};

/*
----------------------------------------------------------------------------------------------------
WARNING!!!
Before making any changes to the code, always review this BNF comment block.
After modifying the code,
**immediately update this BNF comment** to ensure it remains consistent with the implementation.
This comment block must not be removed,
as it serves as the authoritative reference for the language syntax.
----------------------------------------------------------------------------------------------------



program        -> top_level* EOF ;

top_level      -> statement ";"? | function ;

function       -> "function" identifier "(" parameters? ")" block ;
parameters     -> identifier ( "," identifier )* ;

block          -> "{" statement* "}" ;

statement      -> exprStmt
               | varStmt
               | ifStmt
               | returnStmt
               | breakStmt
               | continueStmt
               | repeatStmt
               | whileStmt
               | doUntilStmt
               | forStmt
               | block ;

exprStmt       -> expression terminator ;

varStmt        -> "var" variableDecl ("," variableDecl)* terminator ;
variableDecl   -> IDENTIFIER ("=" expression)? ;

ifStmt         -> "if" ("(" expression ")" | expression) "then"? ifBranch ("else" ifBranch)? ;

ifBranch       -> statement_no_term | block ;

returnStmt     -> "return" expression? terminator ;
breakStmt      -> "break" terminator ;
continueStmt   -> "continue" terminator ;

repeatStmt     -> "repeat" "(" expression ")" statement ;
whileStmt      -> "while" ("(" expression ")" | expression) statement ;
doUntilStmt    -> "do" statement "until" "(" expression ")" terminator ;
forStmt        -> "for" "(" (varStmt_no_term | exprStmt_no_term | ";") expression? ";" (exprStmt_no_term)? ")" statement ;

statement_no_term -> exprStmt_no_term
                  | varStmt_no_term
                  | returnStmt_no_term
                  | breakStmt_no_term
                  | continueStmt_no_term
                  | ifStmt
                  | block ;

exprStmt_no_term     -> expression ;
varStmt_no_term      -> "var" variableDecl ("," variableDecl)* ;
returnStmt_no_term   -> "return" expression? ;
breakStmt_no_term    -> "break" ;
continueStmt_no_term -> "continue" ;

terminator     -> ( ";" | newline )+
---

expression     -> assignment ;

assignment     -> ternary ( ("=" | "+=" | "-=" | "*=" | "/=" | "%=") ternary )? ;

ternary        -> logic_or ( "?" expression ":" ternary )? ;

logic_or       -> logic_xor ( "||" logic_xor )* ;
logic_xor      -> logic_and ( "^^" logic_and )* ;
logic_and      -> bit_or ( "&&" bit_or )* ;
bit_or         -> bit_xor ( "|" bit_xor )* ;
bit_xor        -> bit_and ( "^" bit_and )* ;
bit_and        -> equality ( "&" equality )* ;
equality       -> comparison ( ( "!=" | "==" ) comparison )* ;
comparison     -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           -> factor ( ( "-" | "+" ) factor )* ;
factor         -> postfix ( ( "/" | "*" | "%" ) postfix )* ;
postfix        -> identifier ( "++" | "--" ) | unary ;
unary          -> ( "!" | "~" | "+" | "-" ) unary
               | ( "++" | "--" ) identifier
               | primary ;
primary & atom -> number | string | "true" | "false" | "null"
               | identifier ( "(" ( expression ( "," expression )* )? ")" | ( "++" | "--" ) )?
               | "(" expression ")" ;
*/

/// The top-level parser for a program, parsing a collection of statements and function definitions.
pub(crate) fn program_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Program, extra::Err<Rich<'tokens, Token<'src>>>>
where
    I: ValueInput<'tokens, Token = Token<'src>, Span = SimpleSpan>,
{
    // region terminator
    let terminator = choice((just(Token::Semicolon), just(Token::Newline)))
        .repeated()
        .at_least(1)
        .ignored();
    // endregion

    // region statement
    let expr = expr_parser();

    let statement = recursive(|statement| {
        // region expr_stmt
        let expr_stmt = expr
            .clone()
            .or_not()
            .then_ignore(terminator.clone())
            .map(|expr_opt| expr_opt.map(Stmt::Expr));
        // endregion

        // region var_stmt
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
        // endregion

        // region block
        let block_content = statement
            .clone()
            .repeated()
            .collect::<Vec<Option<Stmt>>>()
            .map(|stmts| stmts.into_iter().flatten().collect::<Vec<Stmt>>());

        let block = block_content
            .clone()
            .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
            .map(|stmts| Some(Stmt::Block(stmts)));
        // endregion

        // region if_stmt
        let if_stmt = recursive(|if_stmt| {
            let block = statement
                .clone()
                .repeated()
                .collect::<Vec<Option<Stmt>>>()
                .map(|stmts| stmts.into_iter().flatten().collect::<Vec<Stmt>>())
                .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
                .map(Stmt::Block);

            let variable_decl = select! { Token::Identifier(s) => s.to_string() }
                .then(just(Token::Equal).ignore_then(expr.clone()).or_not());
            let var_stmt_no_term = just(Token::Var)
                .ignore_then(
                    variable_decl
                        .separated_by(just(Token::Comma))
                        .allow_trailing()
                        .at_least(1)
                        .collect::<Vec<_>>(),
                )
                .map(Stmt::Var);

            let return_stmt_no_term = just(Token::Return)
                .ignore_then(expr.clone().or_not())
                .map(Stmt::Return);

            let break_stmt_no_term = just(Token::Break).map(|_| Stmt::Break);
            let continue_stmt_no_term = just(Token::Continue).map(|_| Stmt::Continue);

            let body = choice((
                block,
                if_stmt,
                var_stmt_no_term,
                return_stmt_no_term,
                break_stmt_no_term,
                continue_stmt_no_term,
                expr.clone().map(Stmt::Expr),
            ));

            just(Token::If)
                .ignore_then(
                    expr.clone()
                        .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                        .or(expr.clone()),
                )
                .then_ignore(just(Token::Then).or_not())
                .then_ignore(just(Token::Newline).repeated())
                .then(body.clone())
                .then_ignore(just(Token::Semicolon).or_not())
                .then_ignore(just(Token::Newline).repeated())
                .then(
                    just(Token::Else)
                        .ignore_then(just(Token::Newline).repeated())
                        .ignore_then(body)
                        .then_ignore(just(Token::Semicolon).or_not())
                        .or_not(),
                )
                .map(|((cond, then_stmt), else_stmt)| {
                    Stmt::If(Box::new(cond), Box::new(then_stmt), else_stmt.map(Box::new))
                })
        });
        // endregion

        // region return_stmt
        let return_stmt = just(Token::Return)
            .ignore_then(expr.clone().or_not())
            .then_ignore(terminator.clone())
            .map(|expr_opt| Some(Stmt::Return(expr_opt)));
        // endregion

        // region break_stmt
        let break_stmt = just(Token::Break)
            .then_ignore(terminator.clone())
            .map(|_| Some(Stmt::Break));
        // endregion

        // region continue_stmt
        let continue_stmt = just(Token::Continue)
            .then_ignore(terminator.clone())
            .map(|_| Some(Stmt::Continue));
        // endregion

        // region repeat_stmt
        let repeat_stmt = just(Token::Repeat)
            .ignore_then(
                expr.clone()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .then_ignore(just(Token::Newline).repeated())
            .then(statement.clone())
            .map(|(count, body)| body.map(|stmt| Stmt::Repeat(Box::new(count), Box::new(stmt))));
        // endregion

        // region while_stmt
        let while_stmt = just(Token::While)
            .ignore_then(
                expr.clone()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen))
                    .or(expr.clone()),
            )
            .then_ignore(just(Token::Newline).repeated())
            .then(statement.clone())
            .map(|(cond, body)| body.map(|stmt| Stmt::While(Box::new(cond), Box::new(stmt))));
        // endregion

        // region do_until_stmt
        let do_until_stmt = just(Token::Do)
            .ignore_then(just(Token::Newline).repeated())
            .ignore_then(statement.clone())
            .then_ignore(just(Token::Newline).repeated())
            .then_ignore(just(Token::Until))
            .then(
                expr.clone()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .then_ignore(terminator.clone())
            .map(|(body, cond)| body.map(|stmt| Stmt::DoUntil(Box::new(stmt), Box::new(cond))));
        // endregion

        // region for_stmt
        let for_stmt = just(Token::For)
            .ignore_then(just(Token::LeftParen))
            .ignore_then(choice((
                just(Token::Var)
                    .ignore_then(
                        select! { Token::Identifier(s) => s.to_string() }
                            .then(just(Token::Equal).ignore_then(expr.clone()).or_not()),
                    )
                    .map(|(name, init)| Some(Box::new(Stmt::Var(vec![(name, init)])))),
                expr.clone().map(|e| Some(Box::new(Stmt::Expr(e)))),
                just(Token::Semicolon).to(None),
            )))
            .then_ignore(just(Token::Semicolon).or_not())
            .then(expr.clone().or_not().map(|e| e.map(Box::new)))
            .then_ignore(just(Token::Semicolon))
            .then(choice((
                expr.clone().map(|e| Some(Box::new(Stmt::Expr(e)))),
                empty().to(None),
            )))
            .then_ignore(just(Token::RightParen))
            .then_ignore(just(Token::Newline).repeated())
            .then(statement.clone())
            .map(|(((init, cond), update), body)| {
                body.map(|stmt| Stmt::For(init, cond, update, Box::new(stmt)))
            });
        // endregion

        choice((
            expr_stmt.clone(),
            var_stmt.clone(),
            if_stmt.map(Some),
            return_stmt.clone(),
            break_stmt.clone(),
            continue_stmt.clone(),
            repeat_stmt.clone(),
            while_stmt.clone(),
            do_until_stmt.clone(),
            for_stmt.clone(),
            block,
        ))
    });
    // endregion

    // region function
    let function_block = statement
        .clone()
        .repeated()
        .collect::<Vec<_>>()
        .map(|stmts| stmts.into_iter().flatten().collect()) // Filter out empty statements
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace));

    let parameters = select! { Token::Identifier(s) => s.to_string() }
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::LeftParen), just(Token::RightParen));

    let function = just(Token::Function)
        .ignore_then(select! { Token::Identifier(s) => s.to_string() })
        .then(parameters)
        .then(function_block)
        .map(|((name, args), body)| {
            TopLevel::Function(FuncDef {
                name,
                func: Func { args, body },
            })
        });
    // endregion

    // region top_level
    let top_level = choice((
        function.map(Some),
        statement.map(|stmt_opt| stmt_opt.map(TopLevel::Statement)),
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
                // Increment/decrement only work on identifiers
                just(Token::Increment)
                    .ignore_then(select! { Token::Identifier(s) => s.to_string() })
                    .map(|id| Expr::PreIncrement(Box::new(Expr::Identifier(id)))),
                just(Token::Decrement)
                    .ignore_then(select! { Token::Identifier(s) => s.to_string() })
                    .map(|id| Expr::PreDecrement(Box::new(Expr::Identifier(id)))),
                atom, // Use atom here instead of the old 'primary'
            ))
        })
        .boxed();
        // endregion

        // region Postfix operators (increment/decrement)
        let postfix = choice((
            // Postfix increment/decrement only work on identifiers
            select! { Token::Identifier(s) => s.to_string() }
                .then(choice((
                    just(Token::Increment).to(Expr::PostIncrement as fn(_) -> _),
                    just(Token::Decrement).to(Expr::PostDecrement as fn(_) -> _),
                )))
                .map(|(id, op)| op(Box::new(Expr::Identifier(id)))),
            // All other unary expressions (without postfix operators)
            unary.clone(),
        ))
        .boxed();
        // endregion

        // region Multiplication, division, modulo
        let factor = postfix
            .clone()
            .foldl(
                choice((
                    just(Token::Star).to(Expr::Multiplication as fn(_, _) -> _),
                    just(Token::Slash).to(Expr::Division as fn(_, _) -> _),
                    just(Token::Percent).to(Expr::Percent as fn(_, _) -> _),
                ))
                .then(postfix)
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
                choice((
                    just(Token::Equal).to(Expr::Equal as fn(_, _) -> _),
                    just(Token::PlusEqual).to(Expr::PlusEqual as fn(_, _) -> _),
                    just(Token::MinusEqual).to(Expr::MinusEqual as fn(_, _) -> _),
                    just(Token::StarEqual).to(Expr::StarEqual as fn(_, _) -> _),
                    just(Token::SlashEqual).to(Expr::SlashEqual as fn(_, _) -> _),
                    just(Token::PercentEqual).to(Expr::PercentEqual as fn(_, _) -> _),
                ))
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
