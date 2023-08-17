use ariadne::{Label, Report, ReportKind};
use std::ops::Range;
use strum_macros::AsRefStr;

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

#[derive(Debug, Clone, PartialEq, AsRefStr)]
pub enum EvaluationError {
  UndefinedVariable(String),
  InvalidOperator(String, String, String),
  InvalidType(String, Vec<String>),
  IndexOutOfBounds(usize, usize),
  IncorrectParameterCount(usize, usize),
  ConstantReassignment(String),
  InvalidRange(f64, f64),
  AssertionFailed,
}

impl ToString for EvaluationError {
  fn to_string(&self) -> String {
    match self {
      EvaluationError::AssertionFailed => self.as_ref().to_string(),
      _ => format!(
        "{}: {}",
        self.as_ref(),
        match self {
          EvaluationError::UndefinedVariable(identifier) => identifier.to_string(),
          EvaluationError::InvalidOperator(operator, left, right) => format!(
            "{:?} {:?} {:?}",
            left,
            operator,
            right, // Do we want to use prefix notation here like in the rest of the language?
          ),
          EvaluationError::InvalidType(found, expected) =>
            format!("found {:?}, expected one of {:?}", found, expected),
          EvaluationError::IndexOutOfBounds(index, length) => format!(
            "index {:?} is not within the range 0..{:?} (inclusive)",
            index, length
          ),
          EvaluationError::IncorrectParameterCount(found, expected) =>
            format!("expected {}, found {}", expected, found),
          EvaluationError::ConstantReassignment(identifier) => identifier.to_string(),
          EvaluationError::InvalidRange(start, end) => format!("{}..{}", start, end),
          _ => unreachable!(),
        }
      ),
    }
  }
}
