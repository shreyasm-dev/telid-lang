use ariadne::{Label, Report, ReportKind};
use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum EvaluationError {
  UndefinedVariable(String),
  InvalidIndex(f64),
  // Operator, left type, right type
  InvalidOperator(String, String, String),
  InvalidType(String, Vec<String>),
  // Index, length
  IndexOutOfBounds(usize, usize),
  // Number of arguments, expected number of arguments
  IncorrectParameterCount(usize, usize),
  Custom(String),
}
