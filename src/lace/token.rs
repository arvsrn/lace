#[derive(Debug, PartialEq)]
pub enum Kind {
    Identifier,
    String,
    Assign,
    Semicolon,
    Colon,
    LeftParen,
    RightParen,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
    GreaterThan,
    LessThan,
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub kind:     Kind,
    pub text:     String,
    pub value:    Option<isize>,
    pub position: (usize, usize)
}

impl Token {
    fn new(kind: Kind, text: String, value: Option<isize>, position: (usize, usize)) -> Self {
        Self {
            kind,
            text,
            value,
            position,
        }
    }
}
