use std::{fmt, ops::Range, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Int(i32),
    Float(f64),
    Identifier(Rc<str>),
    Superscript(Vec<Token>),
    Eq,
    Plus,
    Minus,
    Star,
    Dot,
    Cross,
    Slash,
    Divide,
    Percent,
    Carrot,
    Exclamation,
    Degree,
    Sqrt,
    Cbrt,
    Fort,
    Not,
    EqEq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Pipe,
    LeftFloor,
    RightFloor,
    LeftCeil,
    RightCeil,
    Comma,
    If,
    Else,
    While,
    Newline,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenType::*;
        match self {
            Int(value) => write!(f, "{}", value),
            Float(value) => write!(f, "{}", value),
            Identifier(name) => write!(f, "{}", name),
            Superscript(tokens) => write!(
                f,
                "^({})",
                tokens
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Eq => write!(f, "'='"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Star => write!(f, "'*'"),
            Dot => write!(f, "'∙'"),
            Cross => write!(f, "'×'"),
            Slash => write!(f, "'/'"),
            Divide => write!(f, "'÷'"),
            Percent => write!(f, "'%'"),
            Carrot => write!(f, "'^'"),
            Exclamation => write!(f, "'!'"),
            Degree => write!(f, "'°'"),
            Sqrt => write!(f, "'√'"),
            Cbrt => write!(f, "'∛'"),
            Fort => write!(f, "'∜'"),
            Or => write!(f, "'or'"),
            EqEq => write!(f, "'=='"),
            Neq => write!(f, "'!='"),
            Lt => write!(f, "'<'"),
            Lte => write!(f, "'<='"),
            Gt => write!(f, "'>'"),
            Gte => write!(f, "'>='"),
            Not => write!(f, "'not'"),
            And => write!(f, "'and'"),
            LeftParen => write!(f, "'('"),
            RightParen => write!(f, "')'"),
            LeftBrace => write!(f, "'{{'"),
            RightBrace => write!(f, "'}}'"),
            Pipe => write!(f, "'|'"),
            LeftFloor => write!(f, "'⌊'"),
            RightFloor => write!(f, "'⌋'"),
            LeftCeil => write!(f, "'⌈'"),
            RightCeil => write!(f, "'⌉'"),
            Comma => write!(f, "','"),
            If => write!(f, "'if'"),
            Else => write!(f, "'else'"),
            While => write!(f, "'while'"),
            Newline => write!(f, "'\\n'"),
            EOF => write!(f, "<eof>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub range: Range<usize>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}
