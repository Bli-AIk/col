use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
#[derive(Clone)]
pub(crate) enum Token<'a> {
    // region ---Keywords---
    // See: https://manual.gamemaker.io/monthly/en/#t=GameMaker_Language%2FGML_Overview%2FLanguage_Features.htm&rhsearch=globalvar
    #[token("repeat")]
    Repeat,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("until")]
    Until,
    #[token("for")]
    For,
    #[token("switch")]
    Switch,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("exit")]
    Exit,
    #[token("with")]
    With,
    #[token("return")]
    Return,
    #[token("begin")]
    Begin,
    #[token("end")]
    End,
    #[token("try")]
    Try,
    #[token("catch")]
    Catch,
    #[token("finally")]
    Finally,
    #[token("throw")]
    Throw,
    #[token("new")]
    New,
    #[token("delete")]
    Delete,

    // See Operators:
    // Division and Modulo (div, %, mod)
    #[token("div")]
    Div,
    #[token("mod")]
    Mod,

    // Other
    #[token("var")]
    Var,
    #[token("globalvar")]
    GlobalVar,
    #[token("localvar")]
    LocalVar,
    #[token("function")]
    Function,
    #[token("enum")]
    Enum,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("undefined")]
    Undefined,
    #[token("null")]
    Null,
    #[token("self")]
    Self_,
    #[token("other")]
    Other,
    #[token("and")]
    AndWord,
    #[token("or")]
    OrWord,
    #[token("not")]
    NotWord,
    #[token("global")]
    Global,
    #[token("all")]
    All,
    #[token("noone")]
    Noone,
    #[token("constructor")]
    Constructor,
    #[token("static")]
    Static,

    // ControlFlow
    // See https://manual.gamemaker.io/monthly/en/#t=GameMaker_Language%2FGML_Overview%2FLanguage_Features%2FIf_Else_and_Conditional_Operators.htm&rhsearch=globalvar
    #[token("if")]
    If,
    #[token("else")]
    Else,

    // endregion

    // ----------------------------------------
    // region ---Operators---
    // See: https://manual.gamemaker.io/monthly/en/#t=GameMaker_Language%2FGML_Overview%2FExpressions_And_Operators.htm&rhsearch=globalvar

    // Assigning (=)
    #[token("=")]
    Equal,
    #[token("+=")]
    PlusEqual,
    #[token("-=")]
    MinusEqual,
    #[token("*=")]
    StarEqual,
    #[token("/=")]
    SlashEqual,
    #[token("%=")]
    PercentEqual,

    // Combining (&&, ||, ^^)
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("^^")]
    Xor,

    // Nullish (??, ??=)
    #[token("??=")]
    NullishEqual,
    #[token("??")]
    Nullish,

    // Comparing (<, <=, ==, !=, >, >=)
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,

    // Bitwise (|, &, ^, <<, >>)
    #[token("|")]
    BitOr,
    #[token("&")]
    BitAnd,
    #[token("^")]
    BitXor,
    #[token("<<")]
    ShiftLeft,
    #[token(">>")]
    ShiftRight,

    // Arithmetical (+, -, *, /)
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

    // Increment/Decrement (++, --)
    #[token("++")]
    PlusPlus,
    #[token("--")]
    MinusMinus,

    // Division and Modulo (div, %, mod)
    #[token("%")]
    Percent,
    // Note: 'div' and 'mod' are operators but are parsed as keywords/identifiers in many languages.
    // See Keywords.

    // Unary (!, -, ~)
    #[token("!")]
    Not,
    // Note: Minus is already categorized under Arithmetical.
    #[token("~")]
    BitNot,

    // endregion

    // ----------------------------------------
    // region ---Punctuations---
    #[regex(r"(?:\r\n|\n|\r)+")]
    Newline,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,

    // endregion

    // ----------------------------------------
    // region ---Literals---
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r#""([^"\\]|\\.)*""#)]
    String(&'a str),
    #[regex(r"\d+(\.\d+)?")]
    Number(&'a str),
    // endregion
}

#[cfg(test)]
mod tests {
    use super::*;
    use logos::Logos;
    use owo_colors::OwoColorize;

    /// 通用：带彩色输出的 lex 函数
    fn lex_with_output(input: &'_ str) -> Vec<Token<'_>> {
        let mut lex = Token::lexer(input);
        let mut tokens = Vec::new();
        println!();
        println!("{}", "Test Result :".green());

        while let Some(result) = lex.next() {
            match result {
                Ok(token) => {
                    if token == Token::Newline {
                        println!("{}", "↵ Newline".blue());
                    } else {
                        print!("{:?} ", token);
                    }
                    tokens.push(token);
                }
                Err(_) => {
                    println!("{}", "Lexer error encountered!".red());
                    panic!("Lexer failed on input: {:?}", input);
                }
            }
        }
        println!("\n");
        tokens
    }

    // ---------------------------
    // 分类测试
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
            Token::PlusPlus,
            Token::MinusMinus,
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
        let input = r#"my_ident another123 "hello \"world\"" 42 3.14"#;
        let expected = vec![
            Token::Identifier,
            Token::Identifier,
            Token::String(r#""hello \"world\"""#),
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
            Token::Identifier,
            Token::EqualEqual,
            Token::Number("10"),
            Token::RightParen,
            Token::LeftBrace,
            Token::Newline,
            Token::Identifier,
            Token::PlusEqual,
            Token::Number("1"),
            Token::Semicolon,
            Token::Newline,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Newline,
            Token::Identifier,
            Token::Equal,
            Token::Identifier,
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
