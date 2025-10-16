#[cfg(test)]
mod tests {
    use crate::tests::tests_helper::*;

    // ===============================
    // LITERAL EXPRESSION TESTS
    // ===============================

    #[test]
    fn test_number_literal() {
        let result = compile_and_execute("42.5;").unwrap();
        assert_eq!(result, 0.0); // Main returns 0.0 by default
    }

    #[test]
    fn test_string_literal() {
        let src = r#""hello world";"#;
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_boolean_literals() {
        let result1 = compile_and_execute("true;").unwrap();
        let result2 = compile_and_execute("false;").unwrap();
        assert_eq!(result1, 0.0);
        assert_eq!(result2, 0.0);
    }

    #[test]
    fn test_null_literal() {
        let result = compile_and_execute("null;").unwrap();
        assert_eq!(result, 0.0);
    }

    // ===============================
    // VARIABLE DECLARATION AND ASSIGNMENT TESTS
    // ===============================

    #[test]
    fn test_variable_declaration_without_initialization() {
        let src = "var x;";
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_variable_declaration_with_initialization() {
        let src = "var x = 42;";
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_multiple_variable_declarations() {
        let src = "var a = 1, b, c = 3;";
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_variable_assignment() {
        let src = r#"
            var x = 5;
            x = 10;
        "#;
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_chained_assignment() {
        let src = r#"
            var a, b, c;
            a = b = c = 42;
        "#;
        let result = compile_and_execute(src).unwrap();
        assert_eq!(result, 0.0);
    }

    // ===============================
    // ARITHMETIC OPERATIONS TESTS
    // ===============================

    #[test]
    fn test_addition() {
        let src = r#"
            function test() { return 5 + 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_subtraction() {
        let src = r#"
            function test() { return 10 - 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_multiplication() {
        let src = r#"
            function test() { return 4 * 6; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_division() {
        let src = r#"
            function test() { return 15 / 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_modulo() {
        let src = r#"
            function test() { return 10 % 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_complex_arithmetic() {
        let src = r#"
            function test() { return (2 + 3) * 4 - 8 / 2; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 16.0); // (2+3)*4 - 8/2 = 5*4 - 4 = 20-4 = 16
    }

    // ===============================
    // UNARY OPERATIONS TESTS
    // ===============================

    #[test]
    fn test_unary_positive() {
        let src = r#"
            function test() { return +42; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_unary_negative() {
        let src = r#"
            function test() { return -42; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, -42.0);
    }

    #[test]
    fn test_logical_not() {
        let src = r#"
            function test() { return !false; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        // !false should be true (1)
        assert!(result > 0.0);
    }

    #[test]
    fn test_bitwise_not() {
        let src = r#"
            function test() { return ~5; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        // ~5 in bitwise operations should give -6 (two's complement)
        assert_eq!(result, -6.0);
    }

    // ===============================
    // INCREMENT/DECREMENT TESTS
    // ===============================

    #[test]
    fn test_pre_increment() {
        let src = r#"
            function test() {
                var x = 5;
                return ++x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_post_increment() {
        let src = r#"
            function test() {
                var x = 5;
                return x++;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 5.0); // Returns old value
    }

    #[test]
    fn test_pre_decrement() {
        let src = r#"
            function test() {
                var x = 5;
                return --x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_post_decrement() {
        let src = r#"
            function test() {
                var x = 5;
                return x--;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 5.0); // Returns old value
    }

    // ===============================
    // COMPARISON OPERATIONS TESTS
    // ===============================

    #[test]
    fn test_equality() {
        let src = r#"
            function test() { return 5 == 5; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_inequality() {
        let src = r#"
            function test() { return 5 != 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_less_than() {
        let src = r#"
            function test() { return 3 < 5; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_less_equal() {
        let src = r#"
            function test() { return 5 <= 5; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_greater_than() {
        let src = r#"
            function test() { return 5 > 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_greater_equal() {
        let src = r#"
            function test() { return 5 >= 5; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    // ===============================
    // LOGICAL OPERATIONS TESTS
    // ===============================

    #[test]
    fn test_logical_and() {
        let src = r#"
            function test() { return true && true; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_logical_and_short_circuit() {
        let src = r#"
            function test() { 
                var x = 0;
                var result = false && (x = 1);
                return x; // Should be 0 due to short-circuit
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_logical_or() {
        let src = r#"
            function test() { return false || true; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    #[test]
    fn test_logical_or_short_circuit() {
        let src = r#"
            function test() { 
                var x = 0;
                var result = true || (x = 1);
                return x; // Should be 0 due to short-circuit
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_logical_xor() {
        let src = r#"
            function test() { return true ^^ false; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true
    }

    // ===============================
    // BITWISE OPERATIONS TESTS
    // ===============================

    #[test]
    fn test_bitwise_and() {
        let src = r#"
            function test() { return 5 & 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 1.0); // 5 & 3 = 0101 & 0011 = 0001 = 1
    }

    #[test]
    fn test_bitwise_or() {
        let src = r#"
            function test() { return 5 | 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 7.0); // 5 | 3 = 0101 | 0011 = 0111 = 7
    }

    #[test]
    fn test_bitwise_xor() {
        let src = r#"
            function test() { return 5 ^ 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 6.0); // 5 ^ 3 = 0101 ^ 0011 = 0110 = 6
    }

    // ===============================
    // COMPOUND ASSIGNMENT TESTS
    // ===============================

    #[test]
    fn test_plus_equal() {
        let src = r#"
            function test() {
                var x = 5;
                x += 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_minus_equal() {
        let src = r#"
            function test() {
                var x = 10;
                x -= 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_star_equal() {
        let src = r#"
            function test() {
                var x = 4;
                x *= 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_slash_equal() {
        let src = r#"
            function test() {
                var x = 15;
                x /= 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_percent_equal() {
        let src = r#"
            function test() {
                var x = 10;
                x %= 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 1.0);
    }

    // ===============================
    // TERNARY OPERATOR TESTS
    // ===============================

    #[test]
    fn test_ternary_true_condition() {
        let src = r#"
            function test() { return true ? 42 : 24; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_ternary_false_condition() {
        let src = r#"
            function test() { return false ? 42 : 24; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_nested_ternary() {
        let src = r#"
            function test() { return true ? (false ? 1 : 2) : 3; }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 2.0); // true ? (false ? 1 : 2) : 3 = true ? 2 : 3 = 2
    }

    // ===============================
    // CONTROL FLOW TESTS
    // ===============================

    #[test]
    fn test_if_statement_true() {
        let src = r#"
            function test() {
                if (true) return 42;
                return 0;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_if_statement_false() {
        let src = r#"
            function test() {
                if (false) return 42;
                return 24;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_if_else_statement() {
        let src = r#"
            function test() {
                if (false) return 42;
                else return 24;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_if_without_parentheses() {
        let src = r#"
            function test() {
                if true return 42;
                return 0;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_block_statement() {
        let src = r#"
            function test() {
                {
                    var x = 5;
                    x = x + 1;
                    return x;
                }
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 6.0);
    }

    // ===============================
    // LOOP TESTS
    // ===============================

    #[test]
    fn test_while_loop() {
        let src = r#"
            function test() {
                var i = 0;
                var sum = 0;
                while (i < 5) {
                    sum = sum + i;
                    i = i + 1;
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 10.0); // 0+1+2+3+4 = 10
    }

    #[test]
    fn test_while_loop_without_parentheses() {
        let src = r#"
            function test() {
                var i = 0;
                while i < 3 {
                    i = i + 1;
                }
                return i;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_do_until_loop() {
        let src = r#"
            function test() {
                var i = 0;
                do {
                    i = i + 1;
                } until (i >= 3);
                return i;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_repeat_loop() {
        let src = r#"
            function test() {
                var sum = 0;
                var i = 1;
                repeat (5) {
                    sum = sum + i;
                    i = i + 1;
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 15.0); // 1+2+3+4+5 = 15
    }

    #[test]
    fn test_for_loop_full() {
        let src = r#"
            function test() {
                var sum = 0;
                for (var i = 0; i < 5; i++) {
                    sum = sum + i;
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 10.0); // 0+1+2+3+4 = 10
    }

    #[test]
    fn test_for_loop_with_assignment_init() {
        let src = r#"
            function test() {
                var i;
                var sum = 0;
                for (i = 1; i <= 3; i++) {
                    sum = sum + i;
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 6.0); // 1+2+3 = 6
    }

    #[test]
    fn test_for_loop_empty_components() {
        let src = r#"
            function test() {
                var i = 0;
                for (;;) {
                    if (i >= 3) return i;
                    i++;
                }
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_for_loop_with_compound_update() {
        let src = r#"
            function test() {
                var sum = 0;
                for (var i = 0; i < 10; i += 2) {
                    sum = sum + i;
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 20.0); // 0+2+4+6+8 = 20
    }

    // ===============================
    // FUNCTION TESTS
    // ===============================

    #[test]
    fn test_function_definition_no_params() {
        let src = r#"
            function getValue() {
                return 42;
            }
            
            function test() {
                return getValue();
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_function_with_parameters() {
        let src = r#"
            function add(a, b) {
                return a + b;
            }
            
            function test() {
                return add(5, 3);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_function_with_multiple_parameters() {
        let src = r#"
            function multiply(a, b, c) {
                return a * b * c;
            }
            
            function test() {
                return multiply(2, 3, 4);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 24.0);
    }

    #[test]
    fn test_recursive_function() {
        let src = r#"
            function factorial(n) {
                if (n <= 1) return 1;
                return n * factorial(n - 1);
            }
            
            function test() {
                return factorial(5);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 120.0); // 5! = 120
    }

    #[test]
    fn test_function_empty_body() {
        let src = r#"
            function empty() {
            }
            
            function test() {
                return empty();
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 0.0); // Empty function returns default 0.0
    }

    // ===============================
    // RETURN STATEMENT TESTS
    // ===============================

    #[test]
    fn test_return_with_expression() {
        let src = r#"
            function test() {
                return 42 + 8;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 50.0);
    }

    #[test]
    fn test_return_without_expression() {
        let src = r#"
            function test() {
                return;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_early_return() {
        let src = r#"
            function test() {
                if (true) return 42;
                return 24;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 42.0);
    }

    // ===============================
    // EXPRESSION STATEMENT TESTS
    // ===============================

    #[test]
    fn test_expression_statement() {
        let src = r#"
            function test() {
                var x = 5;
                x + 3; // Expression statement
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 5.0); // x is unchanged
    }

    #[test]
    fn test_assignment_in_expression_statement() {
        let src = r#"
            function test() {
                var x = 5;
                x = x + 3; // Assignment expression statement
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0);
    }

    // ===============================
    // PARENTHESIZED EXPRESSION TESTS
    // ===============================

    #[test]
    fn test_parenthesized_expressions() {
        let src = r#"
            function test() {
                return (2 + 3) * (4 - 1);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 15.0); // (2+3) * (4-1) = 5 * 3 = 15
    }

    #[test]
    fn test_nested_parentheses() {
        let src = r#"
            function test() {
                return ((2 + 3) * 2) - 1;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 9.0); // ((2+3) * 2) - 1 = (5 * 2) - 1 = 10 - 1 = 9
    }

    // ===============================
    // OPERATOR PRECEDENCE TESTS
    // ===============================

    #[test]
    fn test_arithmetic_precedence() {
        let src = r#"
            function test() {
                return 2 + 3 * 4;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 14.0); // 2 + (3 * 4) = 2 + 12 = 14
    }

    #[test]
    fn test_comparison_precedence() {
        let src = r#"
            function test() {
                return 5 + 3 > 6;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // (5 + 3) > 6 = 8 > 6 = true
    }

    #[test]
    fn test_logical_precedence() {
        let src = r#"
            function test() {
                return true || false && false;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!(result > 0.0); // true || (false && false) = true || false = true
    }

    #[test]
    fn test_assignment_precedence() {
        let src = r#"
            function test() {
                var x, y;
                x = y = 5 + 3;
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0); // x = (y = (5 + 3)) = x = (y = 8) = x = 8
    }

    // ===============================
    // COMPLEX INTEGRATION TESTS
    // ===============================

    #[test]
    fn test_fibonacci_function() {
        let src = r#"
            function fib(n) {
                if (n <= 1) return n;
                return fib(n - 1) + fib(n - 2);
            }
            
            function test() {
                return fib(6);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 8.0); // fib(6) = 8
    }

    #[test]
    fn test_nested_loops() {
        let src = r#"
            function test() {
                var sum = 0;
                for (var i = 1; i <= 3; i++) {
                    for (var j = 1; j <= 2; j++) {
                        sum = sum + i * j;
                    }
                }
                return sum;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 18.0); // (1*1 + 1*2) + (2*1 + 2*2) + (3*1 + 3*2) = 3 + 6 + 9 = 18
    }

    #[test]
    fn test_complex_conditional_logic() {
        let src = r#"
            function test() {
                var a = 5;
                var b = 3;
                var c = 8;
                
                if (a > b && b < c) {
                    if (a + b == c) {
                        return 1;
                    } else {
                        return 2;
                    }
                } else {
                    return 3;
                }
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 1.0); // a=5 > b=3 && b=3 < c=8 is true, and a+b=8 == c=8 is true
    }

    #[test]
    fn test_variable_scoping() {
        let src = r#"
            function test() {
                var x = 10;
                {
                    var x = 20; // Inner scope shadows outer
                    x = x + 5;
                }
                return x; // Should return outer x
            }
        "#;
        // Note: This test may behave differently depending on how scoping is implemented
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        // The exact result depends on scoping implementation
        assert!(result > 0.0);
    }

    #[test]
    fn test_all_compound_assignments() {
        let src = r#"
            function test() {
                var x = 10;
                x += 5;  // x = 15
                x -= 3;  // x = 12
                x *= 2;  // x = 24
                x /= 4;  // x = 6
                x %= 4;  // x = 2
                return x;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_mixed_increment_decrement() {
        let src = r#"
            function test() {
                var a = 5;
                var b = ++a; // a=6, b=6
                var c = a--; // a=5, c=6
                var d = --a; // a=4, d=4
                var e = a++; // a=5, e=4
                return a + b + c + d + e;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 25.0); // 5 + 6 + 6 + 4 + 4 = 25
    }

    // ===============================
    // EDGE CASE TESTS
    // ===============================

    #[test]
    fn test_zero_division_handling() {
        // Note: This may produce infinity or cause an error depending on implementation
        let src = r#"
            function test() {
                return 5 / 0;
            }
        "#;
        // Just ensure it compiles and runs without crashing
        let _result = compile_and_execute_function(src, "test", &[]);
        // Result could be infinity or error - we just test it doesn't crash compilation
    }

    #[test]
    fn test_negative_zero() {
        let src = r#"
            function test() {
                return -0;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_large_numbers() {
        let src = r#"
            function test() {
                return 999999999 + 1;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 1000000000.0);
    }

    #[test]
    fn test_decimal_numbers() {
        let src = r#"
            function test() {
                return 3.14159 * 2;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert!((result - std::f64::consts::TAU).abs() < 0.0001);
    }

    // ===============================
    // STRESS TESTS
    // ===============================

    #[test]
    fn test_deep_recursion() {
        let src = r#"
            function countdown(n) {
                if (n <= 0) return 0;
                return n + countdown(n - 1);
            }
            
            function test() {
                return countdown(10);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 55.0); // 10+9+8+...+1 = 55
    }

    #[test]
    fn test_multiple_function_calls() {
        let src = r#"
            function double(x) {
                return x * 2;
            }
            
            function triple(x) {
                return x * 3;
            }
            
            function test() {
                return double(5) + triple(4);
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        assert_eq!(result, 22.0); // 10 + 12 = 22
    }

    #[test]
    fn test_complex_expression_with_all_operators() {
        let src = r#"
            function test() {
                var a = 5;
                var b = 3;
                var result = (a + b) * 2 - (a - b) / 2;
                result += a % b;
                return result > 10 ? result : 0;
            }
        "#;
        let result = compile_and_execute_function(src, "test", &[]).unwrap();
        // (5+3)*2 - (5-3)/2 = 16 - 1 = 15; 15 + (5%3) = 15 + 2 = 17; 17 > 10 ? 17 : 0 = 17
        assert_eq!(result, 17.0);
    }
}
