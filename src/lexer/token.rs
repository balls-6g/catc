#[warn(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // delimiters
    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    // punctuation
    Comma,     // ,
    Colon,     // :
    Semicolon, // ;
    Dot,       // .
    Range,     // ..
    At,        // @

    // identifiers and literals
    // the bare metal version of catlang doesn't have floats
    Identifier(String),
    Int(Ints), // default: I32
    String(String),
    Char(char),
    Bool(bool),

    // keywords
    Use,
    Var,
    Val,
    Fun,
    For,
    In,
    If,
    Else,
    Return,
    Ext,
    Typ,
    Imp,
    Enm,
    SelfKw,
    Goto,
    Lbl,

    // operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    EqEq,
    Neq,
    Lt,         // <
    Le,         // <=
    Gt,         // >
    Ge,         // >=
    Not,        // !
    And,        // &&
    Or,         // ||
    Amp,        // &
    Pipe,       // |>
    Arrow,      // ->
    FatArrow,   // =>
    MatchArrow, // ==>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ints {
    I8(i8),
    I32(i32),
    I64(i64),
    U8(u8),
    U32(u32),
    U64(u64),
}
