#[cfg(test)]
mod tests {
    use crate::parser::expr::{Expr, FuncDef, Program, Stmt, TopLevel};
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
        assert_eq!(p.body.len(), 3);
        assert!(matches!(p.body[0], TopLevel::Statement(Stmt::Var(_))));
        assert!(matches!(p.body[1], TopLevel::Function(_)));
        assert!(matches!(p.body[2], TopLevel::Statement(Stmt::Expr(_))));
    }

    #[test]
    fn block_and_terminators_expr_stmt() {
        let src = "{ 1+2; 3\n 4;;;; }\n";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let block = match &p.body[0] {
            TopLevel::Statement(Stmt::Block(block)) => block,
            _ => panic!("Expected a block statement"),
        };
        assert_eq!(block.len(), 3);
        assert!(matches!(block[0], Stmt::Expr(Expr::Addition(_, _))));
        assert!(matches!(block[1], Stmt::Expr(Expr::Number(3.0))));
        assert!(matches!(block[2], Stmt::Expr(Expr::Number(4.0))));
    }

    #[test]
    fn var_stmt_and_variable_decl_list() {
        let src = "var a=1, b, c=2\n";
        let p = parse_ok(src);
        match &p.body[0] {
            TopLevel::Statement(Stmt::Var(vars)) => {
                assert_eq!(vars.len(), 3);
                assert_eq!(vars[0].0, "a");
                assert!(vars[0].1.is_some());
                assert_eq!(vars[1].0, "b");
                assert!(vars[1].1.is_none());
                assert_eq!(vars[2].0, "c");
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
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 3);

        // if (1) { x = 2; }
        match &p.body[0] {
            TopLevel::Statement(Stmt::If(cond, then_stmt, else_stmt)) => {
                assert!(matches!(**cond, Expr::Number(1.0)));
                assert!(matches!(**then_stmt, Stmt::Block(_)));
                assert!(else_stmt.is_none());
            }
            _ => panic!("Expected if statement"),
        }

        // if 0 then x = 3 else x = 4;
        match &p.body[1] {
            TopLevel::Statement(Stmt::If(cond, then_stmt, else_stmt)) => {
                assert!(matches!(**cond, Expr::Number(0.0)));
                assert!(matches!(**then_stmt, Stmt::Expr(Expr::Equal(_, _))));
                assert!(else_stmt.is_some());
                assert!(matches!(**else_stmt.as_ref().unwrap(), Stmt::Expr(Expr::Equal(_, _))));
            }
            _ => panic!("Expected if statement"),
        }

        // if 1 x = 5 else { x = 6; }
        match &p.body[2] {
            TopLevel::Statement(Stmt::If(cond, then_stmt, else_stmt)) => {
                assert!(matches!(**cond, Expr::Number(1.0)));
                assert!(matches!(**then_stmt, Stmt::Expr(Expr::Equal(_, _))));
                assert!(else_stmt.is_some());
                assert!(matches!(**else_stmt.as_ref().unwrap(), Stmt::Block(_)));
            }
            _ => panic!("Expected if statement"),
        }
    }

    #[test]
    fn if_branch_statement_no_term_variants() {
        let src = r#"
        if 1 return 1 else return
        if 1 break else continue
        if 1 var a=1 else { }
        if 1 x = 1 else y = 2
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 4);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::If(_, _, _))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::If(_, _, _))));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::If(_, _, _))));
        assert!(matches!(&p.body[3], TopLevel::Statement(Stmt::If(_, _, _))));
    }

    #[test]
    fn return_break_continue_with_terminators() {
        let src = "return\n break; continue\n";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 3);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::Return(None))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::Break)));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::Continue)));
    }

    #[test]
    fn repeat_while_do_until() {
        let src = r#"
        repeat(3) x++;
        while (1) { break; }
        while 1 x++;
        do x++; until(0);
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 4);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::Repeat(_, _))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::While(_, _))));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::While(_, _))));
        assert!(matches!(&p.body[3], TopLevel::Statement(Stmt::DoUntil(_, _))));
    }

    #[test]
    fn for_stmt_variants() {
        let src = r#"
        for (var i = 0; i < 3; i++) x += i;
        for (x = 0; x < 1; ) { }
        for (; ; ) break;
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 3);

        // for (var i = 0; i < 3; i++) x += i;
        match &p.body[0] {
            TopLevel::Statement(Stmt::For(init, cond, post, body)) => {
                assert!(init.is_some());
                assert!(matches!(**init.as_ref().unwrap(), Stmt::Var(_)));
                assert!(cond.is_some());
                assert!(matches!(**cond.as_ref().unwrap(), Expr::Less(_, _)));
                assert!(post.is_some());
                assert!(matches!(**post.as_ref().unwrap(), Stmt::Expr(Expr::PostIncrement(_))));
                assert!(matches!(**body, Stmt::Expr(Expr::PlusEqual(_, _))));
            }
            _ => panic!("Expected for statement"),
        }

        // for (x = 0; x < 1; ) { }
        match &p.body[1] {
            TopLevel::Statement(Stmt::For(init, cond, post, body)) => {
                assert!(init.is_some());
                assert!(matches!(**init.as_ref().unwrap(), Stmt::Expr(Expr::Equal(_, _))));
                assert!(cond.is_some());
                assert!(matches!(**cond.as_ref().unwrap(), Expr::Less(_, _)));
                assert!(post.is_none());
                assert!(matches!(**body, Stmt::Block(_)));
            }
            _ => panic!("Expected for statement"),
        }

        // for (; ; ) break;
        match &p.body[2] {
            TopLevel::Statement(Stmt::For(init, cond, post, body)) => {
                assert!(init.is_none());
                assert!(cond.is_none());
                assert!(post.is_none());
                assert!(matches!(**body, Stmt::Break));
            }
            _ => panic!("Expected for statement"),
        }
    }

    #[test]
    fn expression_assignment_and_compound() {
        let src = r#"
        a = 1;
        a += 2; a -= 3; a *= 4; a /= 5; a %= 6;
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 6);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::Expr(Expr::Equal(_, _)))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::Expr(Expr::PlusEqual(_, _)))));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::Expr(Expr::MinusEqual(_, _)))));
        assert!(matches!(&p.body[3], TopLevel::Statement(Stmt::Expr(Expr::StarEqual(_, _)))));
        assert!(matches!(&p.body[4], TopLevel::Statement(Stmt::Expr(Expr::SlashEqual(_, _)))));
        assert!(matches!(&p.body[5], TopLevel::Statement(Stmt::Expr(Expr::PercentEqual(_, _)))));
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
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 5);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::Expr(Expr::Ternary(_, _, _)))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::Expr(Expr::Or(_, _)))));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::Expr(Expr::BitOr(_, _)))));
        assert!(matches!(&p.body[3], TopLevel::Statement(Stmt::Expr(Expr::NotEqual(_, _)))));
        assert!(matches!(&p.body[4], TopLevel::Statement(Stmt::Expr(Expr::Subtraction(_, _)))));
    }

    #[test]
    fn unary_and_postfix_inc_dec() {
        let src = r#"
        !~+-1;
        ++x; --y; x++; y--;
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 5);
        assert!(matches!(&p.body[0], TopLevel::Statement(Stmt::Expr(Expr::Not(_)))));
        assert!(matches!(&p.body[1], TopLevel::Statement(Stmt::Expr(Expr::PreIncrement(_)))));
        assert!(matches!(&p.body[2], TopLevel::Statement(Stmt::Expr(Expr::PreDecrement(_)))));
        assert!(matches!(&p.body[3], TopLevel::Statement(Stmt::Expr(Expr::PostIncrement(_)))));
        assert!(matches!(&p.body[4], TopLevel::Statement(Stmt::Expr(Expr::PostDecrement(_)))));
    }

    #[test]
    fn primary_literals_identifiers_calls_and_parens() {
        let src = r#"
        42; 3.14; "str"; true; false; null; id;
        foo(); bar(1, 2, 3);
        (1 + 2) * 3;
    "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 10);
    }

    #[test]
    fn chained_assignment() {
        let src = "a = b = 1;";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let expr = match &p.body[0] {
            TopLevel::Statement(Stmt::Expr(expr)) => expr,
            _ => panic!("Expected an expression statement"),
        };
        assert!(matches!(expr, Expr::Equal(_, _)));
        if let Expr::Equal(_, right) = expr {
            assert!(matches!(**right, Expr::Equal(_, _)));
        }
    }

    #[test]
    fn nested_ternary() {
        let src = "1 ? 2 : 3 ? 4 : 5;";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let expr = match &p.body[0] {
            TopLevel::Statement(Stmt::Expr(expr)) => expr,
            _ => panic!("Expected an expression statement"),
        };
        assert!(matches!(expr, Expr::Ternary(_, _, _)));
        if let Expr::Ternary(_, _, else_expr) = expr {
            assert!(matches!(**else_expr, Expr::Ternary(_, _, _)));
        }
    }

    #[test]
    fn nested_nested_ternary() {
        let src = "1 ? 2 : 3 ? 4 : 5 ? 6 : 7;";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let expr = match &p.body[0] {
            TopLevel::Statement(Stmt::Expr(expr)) => expr,
            _ => panic!("Expected an expression statement"),
        };
        assert!(matches!(expr, Expr::Ternary(_, _, _)));
        if let Expr::Ternary(_, _, else_expr) = expr {
            assert!(matches!(**else_expr, Expr::Ternary(_, _, _)));
            if let Expr::Ternary(_, _, else_expr2) = &**else_expr {
                assert!(matches!(**else_expr2, Expr::Ternary(_, _, _)));
            }
        }
    }

    #[test]
    fn function_no_params_and_empty_block() {
        let src = "function bar() { }\n";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        match &p.body[0] {
            TopLevel::Function(FuncDef { name, func }) => {
                assert_eq!(name, "bar");
                assert_eq!(func.args.len(), 0);
                assert_eq!(func.body.len(), 0);
            }
            _ => panic!("Expected function definition"),
        }
    }

    #[test]
    fn for_with_compound_update() {
        let src = "for (var i = 0; i < 3; i += 1) x += i;";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        match &p.body[0] {
            TopLevel::Statement(Stmt::For(_, _, post, _)) => {
                assert!(post.is_some());
                assert!(matches!(**post.as_ref().unwrap(), Stmt::Expr(Expr::PlusEqual(_, _))));
            }
            _ => panic!("Expected for statement"),
        }
    }

    #[test]
    fn mixed_prefix_postfix_in_expressions() {
        let src = "a = ++b + --c * d++;";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let expr = match &p.body[0] {
            TopLevel::Statement(Stmt::Expr(expr)) => expr,
            _ => panic!("Expected an expression statement"),
        };
        assert!(matches!(expr, Expr::Equal(_, _)));
    }

    #[test]
    fn postfix_in_call_argument() {
        let src = "foo(id++);";
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 1);
        let expr = match &p.body[0] {
            TopLevel::Statement(Stmt::Expr(expr)) => expr,
            _ => panic!("Expected an expression statement"),
        };
        match expr {
            Expr::Call(name, args) => {
                assert_eq!(name, "foo");
                assert_eq!(args.len(), 1);
                assert!(matches!(args[0], Expr::PostIncrement(_)));
            }
            _ => panic!("Expected call expression"),
        }
    }

    fn parse_err(src: &str) {
        let token_iter = Token::lexer(src).spanned().map(|(tok, span)| match tok {
            Ok(tok) => (tok, span.into()),
            Err(_) => (Token::Error, span.into()),
        });
        let stream =
            Stream::from_iter(token_iter).map((0..src.len()).into(), |(t, s): (_, _)| (t, s));
        let res = program_parser().parse(stream).into_result();
        assert!(res.is_err(), "Expected parse to fail but it succeeded: {}", src);
    }

    #[test]
    fn fail_prefix_on_literal_and_bad_unary_sequences() {
        parse_err("c = ++1;");
        parse_err("c = --1;");

        parse_err("c = +++a;");
        parse_err("c = ---a;");
        parse_err("c = ++-a;");

        parse_err("foo()++;");

        parse_err("a = ;");
        parse_err("var a = ;");

        parse_err("1++;");

        parse_err("a = ? : 1;");
    }

    #[test]
    fn long_program_should_fail_due_to_invalid_unary_inside() {
        let src = r#"
            function big(x, y) {
                var i = 0, acc = 0;
                for (i = 0; i < 10; i++) {
                    if (i % 2 == 0) acc += i;
                    else acc += -+i;
                }
                // The following line contains ++ (illegal) for a literal
                acc += ++1;
                return acc;
            }
            var r = big(1, 2);
        "#;
        parse_err(src);
    }

    #[test]
    fn long_program_ok_complex() {
        let src = r#"
            function sum_and_map(a, b) {
                var i = 0, res = 0;
                for (var k = 0; k < 5; k++) {
                    if (k % 2 == 0) {
                        res += a + k;
                        continue;
                    } else {
                        res += b * (k + 1);
                    }
                }
                // Prefixes and suffixes are mixed with ordinary unary (be careful to avoid producing long symbol strings such as '+++')
                res += -++a; // '-' then '++' identifier -> legal
                res += d++ * ++e; // d++ (postfix)、++e (prefix on identifier) -> legal
                return res ? res : 0;
            }

            function caller() {
                var id = 1;
                foo(id++); // Suffix ++ as function parameters (identifier suffix allowed)
                foo(++id); // Prefix ++ is legal on identifiers
            }

            var total = sum_and_map(1, 2);
        "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 3);
        assert!(matches!(p.body[0], TopLevel::Function(_)));
        assert!(matches!(p.body[1], TopLevel::Function(_)));
        assert!(matches!(p.body[2], TopLevel::Statement(Stmt::Var(_))));
    }

    #[test]
    fn long_program_ok_richer() {
        let src = r#"
            var a = 0, b = 1;
            repeat(3) {
                a++;
                if (a > 1) break;
            }
            do
                b += a;
            until(b > 10);

            while (a < 5) a += 1;

            function nested() {
                {
                    { var x = 1; x = x + 2; }
                }
                return null;
            }
        "#;
        let p = parse_ok(src);
        assert_eq!(p.body.len(), 5);
    }


}
