use crate::error::LexError;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Whitespace,
  Newline,
  Comment,

  NumberLiteral(f64),
  BooleanLiteral(bool),
  StringLiteral(String),
  Void,

  Identifier(String),

  Let,
  Const,
  If,
  Else,
  For,
  In,

  Plus,
  Minus,
  Asterisk,
  Slash,
  Percent,
  Caret,
  Ampersand,
  AmpersandAmpersand,
  Pipe,
  PipePipe,
  EqualsEquals,
  Bang,
  BangEquals,
  LessThan,
  LessThanEquals,
  GreaterThan,
  GreaterThanEquals,

  Equals,

  LeftParen,
  RightParen,
  LeftBrace,
  RightBrace,
  LeftBracket,
  RightBracket,

  Comma,
  Dot,

  Error(LexError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Range<usize>,
}

impl TokenKind {
  pub fn from_identifier(identifier: String) -> Self {
    let identifier = identifier.as_str();
    match identifier {
      "let" => Self::Let,
      "const" => Self::Const,
      "if" => Self::If,
      "else" => Self::Else,
      "for" => Self::For,
      "in" => Self::In,
      "true" => Self::BooleanLiteral(true),
      "false" => Self::BooleanLiteral(false),
      "void" => Self::Void,
      _ => Self::Identifier(identifier.to_string()),
    }
  }
}
