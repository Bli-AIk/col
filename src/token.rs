use logos::Logos;
use owo_colors::OwoColorize;
use std::fmt;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t]+")]
#[logos(skip r"//[^\n]*")]
#[logos(skip r"/\*([^*]|\*[^/])*\*/")]
#[derive(Clone)]
pub(crate) enum Token<'a> {
    Error,
    // region Keywords
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
    #[token("then")]
    Then,
    #[token("else")]
    Else,

    // endregion

    // ----------------------------------------
    // region Operators
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
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,

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
    // region Punctuations
    #[regex(r"(?:\r\n|\n|\r)")]
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
    // region Literals

    // See https://manual.gamemaker.io/lts/en/index.htm#t=GameMaker_Language%2FGML_Overview%2FVariables_And_Variable_Scope.htm
    // Maximum length will be configurable in future
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]{0,63}")]
    Identifier(&'a str),

    // [^"\n]* means that there cannot be " and newline characters in the middle,
    // so only single-line strings are allowed
    #[regex(r#""[^"\n]*""#, |lex| {
    let slice = lex.slice();
    &slice[1..slice.len()-1]
    })]
    String(&'a str),

    #[regex(r"\d+(\.\d+)?")]
    Number(&'a str),
    // endregion
}
impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Error
            Token::Error => write!(f, "Error"),

            // region Keywords
            // ControlFlow
            Token::Repeat => write!(f, "repeat"),
            Token::While => write!(f, "while"),
            Token::Do => write!(f, "do"),
            Token::Until => write!(f, "until"),
            Token::For => write!(f, "for"),
            Token::Switch => write!(f, "switch"),
            Token::Break => write!(f, "break"),
            Token::Continue => write!(f, "continue"),
            Token::Exit => write!(f, "exit"),
            Token::With => write!(f, "with"),
            Token::Return => write!(f, "return"),
            Token::Begin => write!(f, "begin"),
            Token::End => write!(f, "end"),
            Token::Try => write!(f, "try"),
            Token::Catch => write!(f, "catch"),
            Token::Finally => write!(f, "finally"),
            Token::Throw => write!(f, "throw"),
            Token::New => write!(f, "new"),
            Token::Delete => write!(f, "delete"),

            // Division and Modulo (div, %, mod) - Words
            Token::Div => write!(f, "div"),
            Token::Mod => write!(f, "mod"),

            // Other
            Token::Var => write!(f, "var"),
            Token::GlobalVar => write!(f, "globalvar"),
            Token::LocalVar => write!(f, "localvar"),
            Token::Function => write!(f, "function"),
            Token::Enum => write!(f, "enum"),
            Token::Case => write!(f, "case"),
            Token::Default => write!(f, "default"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Undefined => write!(f, "undefined"),
            Token::Null => write!(f, "null"),
            Token::Self_ => write!(f, "self"),
            Token::Other => write!(f, "other"),
            Token::AndWord => write!(f, "and"),
            Token::OrWord => write!(f, "or"),
            Token::NotWord => write!(f, "not"),
            Token::Global => write!(f, "global"),
            Token::All => write!(f, "all"),
            Token::Noone => write!(f, "noone"),
            Token::Constructor => write!(f, "constructor"),
            Token::Static => write!(f, "static"),

            // ControlFlow (If/Else)
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            // endregion

            // ----------------------------------------
            // region Operators
            // Assigning (=, +=, -=, *=, /=, %=)
            Token::Equal => write!(f, "="),
            Token::PlusEqual => write!(f, "+="),
            Token::MinusEqual => write!(f, "-="),
            Token::StarEqual => write!(f, "*="),
            Token::SlashEqual => write!(f, "/="),
            Token::PercentEqual => write!(f, "%="),

            // Combining (&&, ||, ^^)
            Token::And => write!(f, "&&"),
            Token::Or => write!(f, "||"),
            Token::Xor => write!(f, "^^"),

            // Nullish (??, ??=)
            Token::NullishEqual => write!(f, "??="),
            Token::Nullish => write!(f, "??"),

            // Comparing (<, <=, ==, !=, >, >=)
            Token::Less => write!(f, "<"),
            Token::LessEqual => write!(f, "<="),
            Token::EqualEqual => write!(f, "=="),
            Token::NotEqual => write!(f, "!="),
            Token::Greater => write!(f, ">"),
            Token::GreaterEqual => write!(f, ">="),

            // Bitwise (|, &, ^, <<, >>)
            Token::BitOr => write!(f, "|"),
            Token::BitAnd => write!(f, "&"),
            Token::BitXor => write!(f, "^"),
            Token::ShiftLeft => write!(f, "<<"),
            Token::ShiftRight => write!(f, ">>"),

            // Arithmetical (+, -, *, /)
            Token::Increment => write!(f, "++"),
            Token::Decrement => write!(f, "--"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),

            // Division and Modulo (%)
            Token::Percent => write!(f, "%"),
            // Note: 'div' and 'mod' words are in Keywords.

            // Unary (!, ~, - is Arithmetical)
            Token::Not => write!(f, "!"),
            Token::BitNot => write!(f, "~"),
            // endregion

            // ----------------------------------------
            // region Punctuations
            Token::Newline => write!(f, "\\n"),
            Token::Semicolon => write!(f, ";"),
            Token::Comma => write!(f, ","),
            Token::Dot => write!(f, "."),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Question => write!(f, "?"),
            Token::Colon => write!(f, ":"),
            // endregion

            // ----------------------------------------
            // region Literals
            Token::Identifier(s) => write!(f, "{}", s),
            Token::String(s) => write!(f, "{}", s),
            Token::Number(s) => write!(f, "{}", s),
            // endregion
        }
    }
}

pub(crate) fn lex_with_output(input: &'_ str) -> Vec<Token<'_>> {
    let mut lex = Token::lexer(input);
    let mut tokens = Vec::new();
    println!();
    println!("{}", "(Test) Lexer output :".green());

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
#[cfg(test)]
mod tests {
    use super::*;

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
