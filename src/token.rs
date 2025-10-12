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
