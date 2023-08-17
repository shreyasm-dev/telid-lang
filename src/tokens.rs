use crate::{ast::BinaryOperator, error::LexError};
use ordered_float::OrderedFloat;
use std::ops::Range;
use strum_macros::AsRefStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, AsRefStr)]
pub enum TokenKind {
  Whitespace,
  Newline,
  Comment,

  NumberLiteral(OrderedFloat<f64>),
  BooleanLiteral(bool),
  StringLiteral(String),
  Void,

  Identifier(String),

  Let,
  Const,
  Fn,
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
  Semicolon,

  Error(LexError),
  Eof,
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
      "fn" => Self::Fn,
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

  pub fn to_binary_operator(&self) -> BinaryOperator {
    match self {
      Self::Plus => BinaryOperator::Add,
      Self::Minus => BinaryOperator::Subtract,
      Self::Asterisk => BinaryOperator::Multiply,
      Self::Slash => BinaryOperator::Divide,
      Self::Percent => BinaryOperator::Modulo,
      Self::EqualsEquals => BinaryOperator::Equal,
      Self::BangEquals => BinaryOperator::NotEqual,
      Self::LessThan => BinaryOperator::LessThan,
      Self::LessThanEquals => BinaryOperator::LessThanOrEqual,
      Self::GreaterThan => BinaryOperator::GreaterThan,
      Self::GreaterThanEquals => BinaryOperator::GreaterThanOrEqual,
      Self::AmpersandAmpersand => BinaryOperator::And,
      Self::PipePipe => BinaryOperator::Or,
      _ => unreachable!(),
    }
  }
}
