use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]+")]
pub(crate) enum Token {
    // ---Keywords---
    #[token("var")]
    Var,
    #[token("globalvar")]
    GlobalVar,
    #[token("localvar")]
    LocalVar,
    #[token("enum")]
    Enum,
    #[token("function")]
    Function,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("switch")]
    Switch,
    #[token("case")]
    Case,
    #[token("default")]
    Default,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("do")]
    Do,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("undefined")]
    Undefined,
    #[token("null")]
    Null,
    #[token("repeat")]
    Repeat,
    #[token("with")]
    With,
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
    #[token("exit")]
    Exit,
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
    #[token("constructor")]
    Constructor,
    #[token("static")]
    Static,

    // See Operators:
    // Division and Modulo (div, %, mod)
    #[token("div")]
    Div,
    #[token("mod")]
    Mod,

    // ----------------------------------------
    // ---Operators---
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

    // ----------------------------------------
    // Punctuations
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

    // ----------------------------------------
    // ---Literals---
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r#""([^"\\]|\\.)*""#)]
    String,
    #[regex(r"\d+(\.\d+)?")]
    Number,
}
