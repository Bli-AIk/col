#[cfg(test)]
mod tests {
    use crate::parser::expr::{Program, Stmt, TopLevel};
    use crate::parser::program_parser;
    use crate::token::Token;
    use chumsky::{input::Stream, prelude::*};
    use logos::Logos;
    fn parse_ok(src: &str) -> Program {
        let token_iter = Token::lexer(src).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(_) => (Token::Error, span.into()),
        });
        let stream =
            Stream::from_iter(token_iter).map((0..src.len()).into(), |(t, s): (_, _)| (t, s));
        match program_parser().parse(stream).into_result() {
            Ok(p) => p,
            Err(errs) => panic!("Parse failed: {:?}", errs),
        }
    }

    #[test]
    fn program_and_top_level_with_function_and_statement() {
        let src = r#"
        var x = 1; // statement
        function foo(a, b) { return a + b; }
        x = foo(2, 3)
    "#;
        let p = parse_ok(src);
        assert!(p.body.iter().any(|t| matches!(t, TopLevel::Function(_))));
        assert!(p.body.iter().any(|t| matches!(t, TopLevel::Statement(_))));
    }

    #[test]
    fn block_and_terminators_exprstmt() {
        let src = "{ 1+2; 3\n 4;;;; }\n";
        let p = parse_ok(src);
        assert!(matches!(p.body[0], TopLevel::Statement(Stmt::Block(_))));
    }

    #[test]
    fn var_stmt_and_variable_decl_list() {
        let src = "var a=1, b, c=2\n";
        let p = parse_ok(src);
        match &p.body[0] {
            TopLevel::Statement(Stmt::Var(vars)) => {
                assert_eq!(vars.len(), 3);
                assert!(vars[0].1.is_some());
                assert!(vars[1].1.is_none());
                assert!(vars[2].1.is_some());
            }
            _ => panic!("expected var stmt"),
        }
    }

    #[test]
    fn if_stmt_variants_then_else_block_and_no_paren() {
        let src = r#"
        if (1) { x = 2; }
        if 0 then x = 3 else x = 4;
        if 1 x = 5 else { x = 6; }
    "#;
        parse_ok(src);
    }

    #[test]
    fn if_branch_statement_no_term_variants() {
        let src = r#"
        if 1 return 1 else return
        if 1 break else continue
        if 1 var a=1 else { }
        if 1 x = 1 else y = 2
    "#;
        parse_ok(src);
    }

    #[test]
    fn return_break_continue_with_terminators() {
        let src = "return\n break; continue\n";
        parse_ok(src);
    }

    #[test]
    fn repeat_while_do_until() {
        let src = r#"
        repeat(3) x++;
        while (1) { break; }
        while 1 x++;
        do x++; until(0);
    "#;
        parse_ok(src);
    }

    #[test]
    fn for_stmt_variants() {
        let src = r#"
        for (var i = 0; i < 3; i++) x += i;
        for (x = 0; x < 1; ) { }
        for (; ; ) break;
    "#;
        parse_ok(src);
    }

    #[test]
    fn expression_assignment_and_compound() {
        let src = r#"
        a = 1;
        a += 2; a -= 3; a *= 4; a /= 5; a %= 6;
    "#;
        parse_ok(src);
    }

    #[test]
    fn expression_ternary_and_logical_bitwise_comparison_arith() {
        let src = r#"
        1 ? 2 : 3;
        1 || 0 && 1 ^^ 0;
        1 | 2 ^ 3 & 4;
        1 < 2 <= 2 == 2 != 3 > 1 >= 0;
        (1 + 2) * 3 / 4 % 2 - +1;
    "#;
        parse_ok(src);
    }

    #[test]
    fn unary_and_postfix_inc_dec() {
        let src = r#"
        !~+-1;
        ++x; --y; x++; y--;
    "#;
        parse_ok(src);
    }

    #[test]
    fn primary_literals_identifiers_calls_and_parens() {
        let src = r#"
        42; 3.14; "str"; true; false; null; id;
        foo(); bar(1, 2, 3);
        (1 + 2) * 3;
    "#;
        parse_ok(src);
    }
}
