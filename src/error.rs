use ariadne::{Label, Report, ReportKind};
use std::ops::Range;

use crate::tokens::TokenKind;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LexError {
  UnexpectedCharacter(char),
  UnterminatedStringLiteral,
}

impl LexError {
  pub fn report<'a>(
    &'a self,
    src: &'a str,
    span: Range<usize>,
  ) -> Report<'a, (&str, std::ops::Range<usize>)> {
    Report::build(ReportKind::Error, src, span.start)
      .with_message(*self)
      .with_label(Label::new((src, span)))
      .finish()
  }
}

impl ToString for LexError {
  fn to_string(&self) -> String {
    match self {
      LexError::UnexpectedCharacter(c) => format!("Unexpected character: {}", c),
      LexError::UnterminatedStringLiteral => "Unterminated string literal".to_string(),
    }
  }
}

impl Default for LexError {
  fn default() -> Self {
    LexError::UnexpectedCharacter('\0')
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
  pub kind: ParseErrorType,
  pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorType {
  UnexpectedToken(TokenKind, Vec<TokenKind>),
  UnexpectedEnd(Vec<TokenKind>),
}

impl ParseError {
  pub fn new(kind: ParseErrorType, span: Range<usize>) -> Self {
    Self { kind, span }
  }

  pub fn report<'a>(&'a self, src: &'a str) -> Report<'a, (&str, std::ops::Range<usize>)> {
    let message = self.to_string();
    Report::build(ReportKind::Error, src, self.span.start)
      .with_message(message)
      .with_label(Label::new((src, self.span.clone())))
      .finish()
  }
}

impl ToString for ParseError {
  fn to_string(&self) -> String {
    match &self.kind {
      ParseErrorType::UnexpectedToken(token, expected) => format!(
        "Unexpected token: {:?}, expected one of: {:?}",
        token.as_ref(),
        expected.iter().map(|t| t.as_ref()).collect::<Vec<_>>()
      ),
      ParseErrorType::UnexpectedEnd(expected) => {
        format!("Unexpected end of input, expected one of: {:?}", expected)
      }
    }
  }
}
