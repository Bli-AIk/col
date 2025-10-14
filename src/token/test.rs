#[cfg(test)]
mod tests {
    use crate::token::*;

    // ---------------------------
    // ClassificationTest
    // ---------------------------

    #[test]
    fn test_keywords() {
        let input = "\
            repeat while do until for switch break continue exit with return \
            begin end try catch finally throw new delete \
            div mod \
            var globalvar localvar function enum case default true false undefined null self other \
            and or not global all noone constructor static \
            if else";

        let expected = vec![
            Token::Repeat,
            Token::While,
            Token::Do,
            Token::Until,
            Token::For,
            Token::Switch,
            Token::Break,
            Token::Continue,
            Token::Exit,
            Token::With,
            Token::Return,
            Token::Begin,
            Token::End,
            Token::Try,
            Token::Catch,
            Token::Finally,
            Token::Throw,
            Token::New,
            Token::Delete,
            Token::Div,
            Token::Mod,
            Token::Var,
            Token::GlobalVar,
            Token::LocalVar,
            Token::Function,
            Token::Enum,
            Token::Case,
            Token::Default,
            Token::True,
            Token::False,
            Token::Undefined,
            Token::Null,
            Token::Self_,
            Token::Other,
            Token::AndWord,
            Token::OrWord,
            Token::NotWord,
            Token::Global,
            Token::All,
            Token::Noone,
            Token::Constructor,
            Token::Static,
            Token::If,
            Token::Else,
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_operators() {
        let input =
            "= += -= *= /= %= == != < <= > >= ?? ??= && || ^^ | & ^ << >> ++ -- + - * / % ! ~";
        let expected = vec![
            Token::Equal,
            Token::PlusEqual,
            Token::MinusEqual,
            Token::StarEqual,
            Token::SlashEqual,
            Token::PercentEqual,
            Token::EqualEqual,
            Token::NotEqual,
            Token::Less,
            Token::LessEqual,
            Token::Greater,
            Token::GreaterEqual,
            Token::Nullish,
            Token::NullishEqual,
            Token::And,
            Token::Or,
            Token::Xor,
            Token::BitOr,
            Token::BitAnd,
            Token::BitXor,
            Token::ShiftLeft,
            Token::ShiftRight,
            Token::Increment,
            Token::Decrement,
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Percent,
            Token::Not,
            Token::BitNot,
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_punctuations_and_newline() {
        let input = ";\n, . ( ) { } [ ] ? :\r\n";
        let expected = vec![
            Token::Semicolon,
            Token::Newline,
            Token::Comma,
            Token::Dot,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Question,
            Token::Colon,
            Token::Newline,
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_literals_identifiers_and_numbers() {
        let input = r#"my_ident another123 "hello world" 42 3.14"#;
        let expected = vec![
            Token::Identifier("my_ident"),
            Token::Identifier("another123"),
            Token::String("hello world"),
            Token::Number("42"),
            Token::Number("3.14"),
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments_and_newlines() {
        let input = "123 // comment line\n456 /* block comment */ 789\n";
        let expected = vec![
            Token::Number("123"),
            Token::Newline,
            Token::Number("456"),
            Token::Number("789"),
            Token::Newline,
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_complex_snippet() {
        let input = r#"
            if (x == 10) {
                x += 1;
            } else {
                x = x - 1;
            }
        "#;

        let expected = vec![
            Token::Newline,
            Token::If,
            Token::LeftParen,
            Token::Identifier("x"),
            Token::EqualEqual,
            Token::Number("10"),
            Token::RightParen,
            Token::LeftBrace,
            Token::Newline,
            Token::Identifier("x"),
            Token::PlusEqual,
            Token::Number("1"),
            Token::Semicolon,
            Token::Newline,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Newline,
            Token::Identifier("x"),
            Token::Equal,
            Token::Identifier("x"),
            Token::Minus,
            Token::Number("1"),
            Token::Semicolon,
            Token::Newline,
            Token::RightBrace,
            Token::Newline,
        ];

        let tokens = lex_with_output(input);
        assert_eq!(tokens, expected);
    }
}
