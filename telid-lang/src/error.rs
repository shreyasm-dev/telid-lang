use ariadne::{Label, Report, ReportKind};
use std::ops::Range;
use strum_macros::AsRefStr;

use crate::lexer::tokens::TokenKind;

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
pub struct EvaluationError {
  pub kind: EvaluationErrorKind,
  pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum EvaluationErrorKind {
  UndefinedVariable(String),
  InvalidOperator(String, String, String),
  InvalidType(String, Vec<String>),
  IndexOutOfBounds(usize, usize),
  IncorrectParameterCount(usize, usize),
  ConstantReassignment(String),
  InvalidRange(f64, f64),
  AssertionFailed,
}

impl EvaluationError {
  pub fn report<'a>(
    &'a self,
    src: &'a str,
    span: Range<usize>,
    tokens: Vec<(TokenKind, Range<usize>)>,
  ) -> Report<'a, (&str, std::ops::Range<usize>)> {
    let span = tokens[span.start].1.start..tokens[span.end - 1].1.end;
    Report::build(ReportKind::Error, src, span.start)
      .with_message(self.kind.to_string())
      .with_label(Label::new((src, span)))
      .finish()
  }
}

impl ToString for EvaluationErrorKind {
  fn to_string(&self) -> String {
    match self {
      EvaluationErrorKind::AssertionFailed => self.as_ref().to_string(),
      _ => format!(
        "{}: {}",
        self.as_ref(),
        match self {
          EvaluationErrorKind::UndefinedVariable(identifier) => identifier.to_string(),
          EvaluationErrorKind::InvalidOperator(operator, left, right) => format!(
            "{:?} {:?} {:?}",
            left,
            operator,
            right, // Do we want to use prefix notation here like in the rest of the language?
          ),
          EvaluationErrorKind::InvalidType(found, expected) =>
            format!("found {:?}, expected one of {:?}", found, expected),
          EvaluationErrorKind::IndexOutOfBounds(index, length) => format!(
            "index {:?} is not within the range [0..{:?})",
            index, length
          ),
          EvaluationErrorKind::IncorrectParameterCount(found, expected) =>
            format!("expected {}, found {}", expected, found),
          EvaluationErrorKind::ConstantReassignment(identifier) => identifier.to_string(),
          EvaluationErrorKind::InvalidRange(start, end) => format!("{}..{}", start, end),
          _ => unreachable!(),
        }
      ),
    }
  }
}
