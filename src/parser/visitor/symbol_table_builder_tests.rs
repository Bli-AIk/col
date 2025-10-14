#[cfg(test)]
mod tests {
    use crate::parser::program::Program;
    use crate::parser::program_parser;
    use crate::parser::visitor::Visitor;
    use crate::parser::visitor::symbol_table_builder::{Scope, Symbol, SymbolTableBuilder};
    use crate::token::Token;
    use chumsky::{input::Stream, prelude::*};
    use logos::Logos;

    fn parse_gml(src: &str) -> Program {
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
    fn test_basic_variable_and_function_symbols() {
        let src = r#"
            var x = 42;
            var y;
            function test_func(a, b) {
                var local_var = a;
                return a + b;
            }
            test_func(x, y);
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Check global variables
        assert!(scope.table.contains_key("x"));
        assert!(scope.table.contains_key("y"));
        assert!(matches!(scope.table.get("x"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("y"), Some(Symbol::Variable)));

        // Check function
        assert!(scope.table.contains_key("test_func"));
        if let Some(Symbol::Function { parameters }) = scope.table.get("test_func") {
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0], "a");
            assert_eq!(parameters[1], "b");
        } else {
            panic!("Expected function symbol for test_func");
        }

        // Check function scope
        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];

        // Function parameters should be in function scope
        assert!(func_scope.table.contains_key("a"));
        assert!(func_scope.table.contains_key("b"));
        assert!(func_scope.table.contains_key("local_var"));
        assert!(matches!(func_scope.table.get("a"), Some(Symbol::Variable)));
        assert!(matches!(func_scope.table.get("b"), Some(Symbol::Variable)));
        assert!(matches!(
            func_scope.table.get("local_var"),
            Some(Symbol::Variable)
        ));
    }

    #[test]
    fn test_nested_scopes() {
        let src = r#"
            var global_var = 1;
            
            if (true) {
                var if_var = 2;
                if (false) {
                    var nested_if_var;
                } else {
                    var else_var;
                }
            }
            
            while (1) {
                var while_var;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Check global variable
        assert!(scope.table.contains_key("global_var"));
        assert!(matches!(
            scope.table.get("global_var"),
            Some(Symbol::Variable)
        ));

        // Should have 2 child scopes: if statement then branch and while
        assert_eq!(scope.children.len(), 2);

        // Check outer if statement scope (then branch)
        let outer_if_scope = &scope.children[0];
        // The if statement has a Block as then branch, so it creates a scope for the block
        assert_eq!(outer_if_scope.children.len(), 1);
        let if_block_scope = &outer_if_scope.children[0];
        assert!(if_block_scope.table.contains_key("if_var"));
        assert!(matches!(
            if_block_scope.table.get("if_var"),
            Some(Symbol::Variable)
        ));

        // If block should have 2 child scopes: nested if then and else branches
        assert_eq!(if_block_scope.children.len(), 2);

        // Nested if then branch creates a scope, then the block creates another
        let nested_if_then_scope = &if_block_scope.children[0];
        assert_eq!(nested_if_then_scope.children.len(), 1);
        let nested_if_block_scope = &nested_if_then_scope.children[0];
        assert!(nested_if_block_scope.table.contains_key("nested_if_var"));
        assert!(matches!(
            nested_if_block_scope.table.get("nested_if_var"),
            Some(Symbol::Variable)
        ));

        // Nested if else branch creates a scope, then the block creates another
        let nested_else_scope = &if_block_scope.children[1];
        assert_eq!(nested_else_scope.children.len(), 1);
        let nested_else_block_scope = &nested_else_scope.children[0];
        assert!(nested_else_block_scope.table.contains_key("else_var"));
        assert!(matches!(
            nested_else_block_scope.table.get("else_var"),
            Some(Symbol::Variable)
        ));

        // Check while scope
        let while_scope = &scope.children[1];
        // While body is a block, so it creates a nested scope
        assert_eq!(while_scope.children.len(), 1);
        let while_body_scope = &while_scope.children[0];
        assert!(while_body_scope.table.contains_key("while_var"));
        assert!(matches!(
            while_body_scope.table.get("while_var"),
            Some(Symbol::Variable)
        ));
    }

    #[test]
    fn test_loop_scopes() {
        let src = r#"
            for (var i = 0; i < 10; i++) {
                var for_var;
            }
            repeat (3) {
                var repeat_var;
            }
            do {
                var do_var;
            } until (true);
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Should have 3 child scopes: for, repeat, do-until
        assert_eq!(scope.children.len(), 3);

        // Check for loop scope
        let for_scope = &scope.children[0];
        assert!(for_scope.table.contains_key("i"));
        assert!(matches!(for_scope.table.get("i"), Some(Symbol::Variable)));
        // The for body is a block, so it creates a nested scope
        assert_eq!(for_scope.children.len(), 1);
        let for_body_scope = &for_scope.children[0];
        assert!(for_body_scope.table.contains_key("for_var"));
        assert!(matches!(
            for_body_scope.table.get("for_var"),
            Some(Symbol::Variable)
        ));

        // Check repeat scope
        let repeat_scope = &scope.children[1];
        // Repeat body is a block, so it creates a nested scope
        assert_eq!(repeat_scope.children.len(), 1);
        let repeat_body_scope = &repeat_scope.children[0];
        assert!(repeat_body_scope.table.contains_key("repeat_var"));
        assert!(matches!(
            repeat_body_scope.table.get("repeat_var"),
            Some(Symbol::Variable)
        ));

        // Check do-until scope
        let do_scope = &scope.children[2];
        // Do-until body is a block, so it creates a nested scope
        assert_eq!(do_scope.children.len(), 1);
        let do_body_scope = &do_scope.children[0];
        assert!(do_body_scope.table.contains_key("do_var"));
        assert!(matches!(
            do_body_scope.table.get("do_var"),
            Some(Symbol::Variable)
        ));
    }

    #[test]
    fn test_empty_program() {
        let src = "";
        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.is_empty());
        assert!(scope.children.is_empty());
    }

    #[test]
    fn test_function_with_no_parameters() {
        let src = r#"
            function no_params() {
                return 42;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.contains_key("no_params"));
        if let Some(Symbol::Function { parameters }) = scope.table.get("no_params") {
            assert!(parameters.is_empty());
        } else {
            panic!("Expected function symbol for no_params");
        }

        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];
        assert!(func_scope.table.is_empty());
    }

    #[test]
    fn test_multiple_variable_declarations() {
        let src = r#"
            var var1 = 1, var2, var3 = "test";
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.contains_key("var1"));
        assert!(scope.table.contains_key("var2"));
        assert!(scope.table.contains_key("var3"));
        assert!(matches!(scope.table.get("var1"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("var2"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("var3"), Some(Symbol::Variable)));
    }

    #[test]
    fn test_break_and_continue_statements() {
        let src = r#"
            while (true) {
                break;
                continue;
                var loop_var;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert_eq!(scope.children.len(), 1);
        let while_scope = &scope.children[0];
        // While creates a scope, and then Block creates another nested scope
        assert_eq!(while_scope.children.len(), 1);
        let block_scope = &while_scope.children[0];
        assert!(block_scope.table.contains_key("loop_var"));
        assert!(matches!(
            block_scope.table.get("loop_var"),
            Some(Symbol::Variable)
        ));
    }

    #[test]
    fn test_expression_statements() {
        let src = r#"
            var x = 1;
            x = 2;
            someFunc(x);
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.contains_key("x"));
        assert!(matches!(scope.table.get("x"), Some(Symbol::Variable)));
        // Note: someFunc is called but not defined in this program, so it won't be in the symbol table
        assert!(!scope.table.contains_key("someFunc"));
    }

    #[test]
    fn test_return_statements() {
        let src = r#"
            function test_returns(param) {
                return;
                return param;
                return 42;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.contains_key("test_returns"));
        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];
        assert!(func_scope.table.contains_key("param"));
    }

    #[test]
    fn test_sample_gml_file() {
        let src = r#"
            var a = 1;
            var b, c = 2;

            function my_func(arg1, arg2) {
                var d = 3;
                return d + arg1;
            }

            var e = my_func(a, c);
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Check global variables
        assert!(scope.table.contains_key("a"));
        assert!(scope.table.contains_key("b"));
        assert!(scope.table.contains_key("c"));
        assert!(scope.table.contains_key("e"));
        assert!(matches!(scope.table.get("a"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("b"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("c"), Some(Symbol::Variable)));
        assert!(matches!(scope.table.get("e"), Some(Symbol::Variable)));

        // Check function
        assert!(scope.table.contains_key("my_func"));
        if let Some(Symbol::Function { parameters }) = scope.table.get("my_func") {
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0], "arg1");
            assert_eq!(parameters[1], "arg2");
        } else {
            panic!("Expected function symbol for my_func");
        }

        // Check function scope
        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];
        assert!(func_scope.table.contains_key("arg1"));
        assert!(func_scope.table.contains_key("arg2"));
        assert!(func_scope.table.contains_key("d"));
    }

    #[test]
    fn test_complex_nested_structure() {
        let src = r#"
            var my_global = 0;
            
            function complex_func(x, y) {
                if (x > 0) {
                    var then_var = x;
                } else {
                    var else_var;
                }
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Check global level
        assert!(scope.table.contains_key("my_global"));
        assert!(scope.table.contains_key("complex_func"));

        // Check function scope (should have 1 child: the function)
        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];
        assert!(func_scope.table.contains_key("x"));
        assert!(func_scope.table.contains_key("y"));

        // Function should have 2 child scopes: then and else branches of the if statement
        assert_eq!(func_scope.children.len(), 2);

        // Then branch: if creates a scope, then the block creates another nested scope
        let then_scope = &func_scope.children[0];
        assert_eq!(then_scope.children.len(), 1);
        let then_block_scope = &then_scope.children[0];
        assert!(then_block_scope.table.contains_key("then_var"));

        // Else branch: if creates a scope, then the block creates another nested scope
        let else_scope = &func_scope.children[1];
        assert_eq!(else_scope.children.len(), 1);
        let else_block_scope = &else_scope.children[0];
        assert!(else_block_scope.table.contains_key("else_var"));
    }

    #[test]
    fn test_realistic_gml_code() {
        let src = r#"
            var counter = 0;
            var max_count = 10;
            
            function increment_counter() {
                counter++;
                return counter;
            }
            
            function process_data(data, threshold) {
                var result = 0;
                var temp;
                
                for (var i = 0; i < 5; i++) {
                    if (data > threshold) {
                        temp = data * 2;
                        result += temp;
                    } else {
                        temp = data / 2;
                        result -= temp;
                    }
                }
                
                while (result > 100) {
                    result = result / 2;
                }
                
                return result;
            }
            
            var final_result = process_data(increment_counter(), max_count);
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Check global variables
        assert!(scope.table.contains_key("counter"));
        assert!(scope.table.contains_key("max_count"));
        assert!(scope.table.contains_key("final_result"));

        // Check functions
        assert!(scope.table.contains_key("increment_counter"));
        assert!(scope.table.contains_key("process_data"));

        if let Some(Symbol::Function { parameters }) = scope.table.get("process_data") {
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0], "data");
            assert_eq!(parameters[1], "threshold");
        } else {
            panic!("Expected function symbol for process_data");
        }

        // Should have 2 function scopes
        assert_eq!(scope.children.len(), 2);

        // Check process_data function scope
        let process_data_scope = &scope.children[1];
        assert!(process_data_scope.table.contains_key("data"));
        assert!(process_data_scope.table.contains_key("threshold"));
        assert!(process_data_scope.table.contains_key("result"));
        assert!(process_data_scope.table.contains_key("temp"));
    }

    #[test]
    fn test_postfix_operators() {
        let src = r#"
            var x = 5;
            x++;
            ++x;
            x--;
            --x;
            
            function test_ops(y) {
                var z = y++;
                return ++z;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        assert!(scope.table.contains_key("x"));
        assert!(scope.table.contains_key("test_ops"));

        // Check function scope
        assert_eq!(scope.children.len(), 1);
        let func_scope = &scope.children[0];
        assert!(func_scope.table.contains_key("y"));
        assert!(func_scope.table.contains_key("z"));
    }

    #[test]
    fn test_if_with_block_statements() {
        let src = r#"
            if (true) {
                var block_var1 = 1;
                var block_var2;
            } else {
                var else_block_var;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);

        builder.visit_program(&program);

        // Should have 2 child scopes: then and else branches
        assert_eq!(scope.children.len(), 2);

        // Then branch: if creates a scope, then block creates another nested scope
        let then_scope = &scope.children[0];
        assert_eq!(then_scope.children.len(), 1);
        let then_block_scope = &then_scope.children[0];
        assert!(then_block_scope.table.contains_key("block_var1"));
        assert!(then_block_scope.table.contains_key("block_var2"));

        // Else branch: if creates a scope, then block creates another nested scope
        let else_scope = &scope.children[1];
        assert_eq!(else_scope.children.len(), 1);
        let else_block_scope = &else_scope.children[0];
        assert!(else_block_scope.table.contains_key("else_block_var"));
    }

    #[test]
    fn test_complex_mixed_nesting() {
        let src = r#"
            var g = 0;

            if (true) {
                for (var i = 0; i < 3; i++) {
                    var a = i;
                    while (a < 5) {
                        if (a == 2) {
                            var inner1;
                        } else {
                            repeat (2) {
                                var inner2;
                                do {
                                    var inner3;
                                } until (a > 10);
                            }
                        }
                        a++;
                    }
                }
            } else {
                var else_top;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // top-level globals
        assert!(scope.table.contains_key("g"));

        // top-level should have 2 children for the if (then + else)
        assert_eq!(scope.children.len(), 2);

        // THEN branch
        let then_scope = &scope.children[0];
        // then creates a block child
        assert_eq!(then_scope.children.len(), 1);
        let then_block_scope = &then_scope.children[0];

        // inside then-block there should be a for -> for creates its own child
        assert_eq!(then_block_scope.children.len(), 1);
        let for_scope = &then_block_scope.children[0];
        assert!(for_scope.table.contains_key("i"));
        // for body is a block
        assert_eq!(for_scope.children.len(), 1);
        let for_body_scope = &for_scope.children[0];
        assert!(for_body_scope.table.contains_key("a"));

        // for body should contain a while -> while creates a child
        assert_eq!(for_body_scope.children.len(), 1);
        let while_scope = &for_body_scope.children[0];
        // while body is a block
        assert_eq!(while_scope.children.len(), 1);
        let while_body_scope = &while_scope.children[0];

        // while body contains an if which creates two children: then & else
        assert_eq!(while_body_scope.children.len(), 2);

        // while->if->then branch
        let if_then_scope = &while_body_scope.children[0];
        assert_eq!(if_then_scope.children.len(), 1);
        let if_then_block = &if_then_scope.children[0];
        assert!(if_then_block.table.contains_key("inner1"));

        // while->if->else branch
        let if_else_scope = &while_body_scope.children[1];
        assert_eq!(if_else_scope.children.len(), 1);
        let if_else_block = &if_else_scope.children[0];

        // else block contains a repeat -> repeat has a body child
        assert_eq!(if_else_block.children.len(), 1);
        let repeat_scope = &if_else_block.children[0];
        assert_eq!(repeat_scope.children.len(), 1);
        let repeat_body = &repeat_scope.children[0];
        assert!(repeat_body.table.contains_key("inner2"));

        // repeat body contains a do-until -> do creates a scope with a block child
        assert_eq!(repeat_body.children.len(), 1);
        let do_scope = &repeat_body.children[0];
        assert_eq!(do_scope.children.len(), 1);
        let do_block = &do_scope.children[0];
        assert!(do_block.table.contains_key("inner3"));

        // ELSE branch (top-level) should contain else_top
        let else_scope = &scope.children[1];
        assert_eq!(else_scope.children.len(), 1);
        let else_block = &else_scope.children[0];
        assert!(else_block.table.contains_key("else_top"));
    }

    #[test]
    fn test_repeat_do_for_nesting() {
        let src = r#"
            repeat (2) {
                var r1;
                if (true) {
                    do {
                        var d1;
                        for (var j = 0; j < 2; j++) {
                            var fj;
                        }
                    } until (false);
                }
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // top-level: one repeat
        assert_eq!(scope.children.len(), 1);
        let repeat_scope = &scope.children[0];
        // repeat has a body block
        assert_eq!(repeat_scope.children.len(), 1);
        let repeat_body = &repeat_scope.children[0];
        assert!(repeat_body.table.contains_key("r1"));

        // repeat body has an if -> two children (then only used here)
        // But since only then branch exists here, we still expect one child for the if
        // The builder's pattern: if creates one scope and then block as its child
        assert!(!repeat_body.children.is_empty());
        let if_scope = &repeat_body.children[0];
        assert_eq!(if_scope.children.len(), 1);
        let if_block = &if_scope.children[0];

        // inside if-block there is a do-until -> creates a do scope with a block child
        assert_eq!(if_block.children.len(), 1);
        let do_scope = &if_block.children[0];
        assert_eq!(do_scope.children.len(), 1);
        let do_block = &do_scope.children[0];
        assert!(do_block.table.contains_key("d1"));

        // inside do-block there is a for -> for creates a scope; for body contains fj
        assert_eq!(do_block.children.len(), 1);
        let for_scope = &do_block.children[0];
        assert!(for_scope.table.contains_key("j"));
        assert_eq!(for_scope.children.len(), 1);
        let for_body = &for_scope.children[0];
        assert!(for_body.table.contains_key("fj"));
    }

    #[test]
    fn test_deep_alternating_loops_and_conditionals() {
        let src = r#"
            var top = 1;
            while (top < 3) {
                var w = top;
                if (w % 2 == 0) {
                    for (var k = 0; k < 2; k++) {
                        var fk;
                        repeat (1) {
                            var rr;
                        }
                    }
                } else {
                    do {
                        var dd;
                        while (dd < 5) {
                            var inner_w;
                            break;
                        }
                    } until (top > 10);
                }
                top++;
            }
        "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // globals
        assert!(scope.table.contains_key("top"));

        // top-level while -> one child
        assert_eq!(scope.children.len(), 1);
        let while_scope = &scope.children[0];
        assert_eq!(while_scope.children.len(), 1);
        let while_body = &while_scope.children[0];
        assert!(while_body.table.contains_key("w"));

        // while body contains an if -> should have two children (then & else)
        assert_eq!(while_body.children.len(), 2);

        // then branch: for -> for body -> repeat -> repeat body contains rr
        let then_branch = &while_body.children[0];
        assert_eq!(then_branch.children.len(), 1);
        let then_block = &then_branch.children[0];
        assert_eq!(then_block.children.len(), 1);
        let for_scope = &then_block.children[0];
        assert!(for_scope.table.contains_key("k"));
        assert_eq!(for_scope.children.len(), 1);
        let for_body = &for_scope.children[0];
        assert!(for_body.table.contains_key("fk"));
        assert_eq!(for_body.children.len(), 1);
        let repeat_scope = &for_body.children[0];
        assert_eq!(repeat_scope.children.len(), 1);
        let repeat_body = &repeat_scope.children[0];
        assert!(repeat_body.table.contains_key("rr"));

        // else branch: do -> do body contains dd and a nested while with inner_w
        let else_branch = &while_body.children[1];
        assert_eq!(else_branch.children.len(), 1);
        let else_block = &else_branch.children[0];
        assert_eq!(else_block.children.len(), 1);
        let do_scope = &else_block.children[0];
        assert_eq!(do_scope.children.len(), 1);
        let do_block = &do_scope.children[0];
        assert!(do_block.table.contains_key("dd"));

        // do-block contains a while
        assert_eq!(do_block.children.len(), 1);
        let inner_while = &do_block.children[0];
        assert_eq!(inner_while.children.len(), 1);
        let inner_while_body = &inner_while.children[0];
        assert!(inner_while_body.table.contains_key("inner_w"));
    }

    // Error / boundary case tests (append the following directly to your tests module)

    #[test]
    fn test_redeclaration_same_scope_is_handled() {
        // Redeclaring a variable with the same name in the same scope:
        // Key point: builder should not panic, and the symbol table should at least contain the name
        let src = r#"
        var a = 1;
        var a;
        var b = 2;
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // Only assert that it does not crash and that the names are recorded
        assert!(scope.table.contains_key("a"));
        assert!(scope.table.contains_key("b"));
    }

    #[test]
    fn test_shadowing_inner_scope() {
        // Outer scope has x, inner scope redeclares x (shadowing)
        let src = r#"
        var x = 10;
        if (true) {
            var x = 20;
            var inner_only = 1;
        }
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // Top-level must have x
        assert!(scope.table.contains_key("x"));

        // The if block creates a scope (or a series of child nodes),
        // we only check that inner_only and x exist in some child scope
        let mut found_inner_x = false;
        let mut found_inner_only = false;
        for child in &scope.children {
            // Look into the if -> block hierarchy
            for g in &child.children {
                if g.table.contains_key("inner_only") {
                    found_inner_only = true;
                }
                if g.table.contains_key("x") && g.table.get("x").is_some() {
                    found_inner_x = true;
                }
            }
        }

        assert!(
            found_inner_only,
            "Inner variable inner_only should exist in some child scope"
        );
        assert!(
            found_inner_x,
            "Inner shadowed x should exist in some child scope"
        );
    }

    #[test]
    fn test_redeclaration_in_different_scopes() {
        // Same name declared in different scopes should coexist (parent and inner scopes each record it)
        let src = r#"
        var name = 1;
        if (true) {
            var name = 2;
        }
        function f() {
            var name = 3;
        }
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // Top-level has name and function f
        assert!(scope.table.contains_key("name"));
        assert!(scope.table.contains_key("f"));

        // At least one inner scope should have another name
        let mut found_inner_name = false;
        for child in &scope.children {
            if child.table.contains_key("name") {
                found_inner_name = true;
            }
            for grand in &child.children {
                if grand.table.contains_key("name") {
                    found_inner_name = true;
                }
            }
        }
        assert!(
            found_inner_name,
            "At least one child scope should contain another 'name'"
        );
    }

    #[test]
    fn test_duplicate_function_names() {
        // Two functions with the same name: ensure builder does not panic and at least records the name
        let src = r#"
        function dup() { return 1; }
        function dup(a, b) { return a + b; }
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        assert!(scope.table.contains_key("dup"));
        // If builder stores function parameters, check it's a Function type (not enforcing parameter count)
        if let Some(sym) = scope.table.get("dup") {
            match sym {
                Symbol::Function { parameters } => {
                    // Just check that parameters vector exists
                    let _ = parameters.len();
                }
                _ => panic!("dup should be a Function type or recorded as a function"),
            }
        }
    }

    #[test]
    fn test_duplicate_parameter_names_in_function() {
        // Duplicate parameter names: ensure builder does not panic
        let src = r#"
        function weird(a, a, b) {
            var v = a;
        }
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        assert!(scope.table.contains_key("weird"));
        // If parameters are stored as a vec, check length >= 1 (just ensure no panic)
        if let Some(Symbol::Function { parameters }) = scope.table.get("weird") {
            assert!(
                parameters.len() >= 1,
                "Parameters vector should be recorded at least once (even if duplicated)"
            );
        } else {
            panic!("weird is not recorded as a function symbol");
        }
    }

    #[test]
    fn test_empty_blocks_and_nested_empty_blocks() {
        // Empty blocks and nested empty blocks (should not cause child index errors or panic)
        let src = r#"
        if (true) { { { } } }
        while (0) { {} }
        repeat (1) { {} }
        function empty_blocks() { { } }
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        // Key point: visiting the program should not panic, and symbol table structure should be valid
        builder.visit_program(&program);

        // If it reaches here, no panic occurred; also check the function name exists
        assert!(scope.table.contains_key("empty_blocks"));
    }

    #[test]
    fn test_var_decl_with_many_commas() {
        // Multiple variable declarations, mixing initialized and uninitialized (likely to expose parsing/symbol addition boundaries)
        let src = r#"
        var a, b = 2, c, d = 4, e;
    "#;

        let program = parse_gml(src);
        let mut scope = Scope::new();
        let mut builder = SymbolTableBuilder::new(&mut scope);
        builder.visit_program(&program);

        // Pass if all names are recorded
        for name in &["a", "b", "c", "d", "e"] {
            assert!(
                scope.table.contains_key(*name),
                "Expected variable {} to exist",
                name
            );
        }
    }
}
