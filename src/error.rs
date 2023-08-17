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
  InvalidOperator(String, String, String),
  InvalidType(String, Vec<String>),
  IndexOutOfBounds(usize, usize),
  IncorrectParameterCount(usize, usize),
  ConstantReassignment(String),
  InvalidRange(f64, f64),
}

impl ToString for EvaluationError {
  fn to_string(&self) -> String {
    match self {
      EvaluationError::UndefinedVariable(identifier) => {
        format!("Undefined variable: {:?}", identifier)
      }
      EvaluationError::InvalidOperator(operator, left, right) => format!(
        "Invalid operator for types {:?} and {:?}: {:?}",
        left, right, operator
      ),
      EvaluationError::InvalidType(found, expected) => {
        format!(
          "Invalid type: found {:?}, expected one of {:?}",
          found, expected
        )
      }
      EvaluationError::IndexOutOfBounds(index, length) => format!(
        "Index out of bounds: index {:?} is not within the range 0..{:?} (inclusive)",
        index, length
      ),
      EvaluationError::IncorrectParameterCount(found, expected) => format!(
        "Incorrect number of parameters: expected {}, found {}",
        expected, found
      ),
      EvaluationError::ConstantReassignment(identifier) => {
        format!("Cannot reassign constant: {}", identifier)
      }
      EvaluationError::InvalidRange(start, end) => {
        format!("Invalid range: {}..{}", start, end)
      }
    }
  }
}
