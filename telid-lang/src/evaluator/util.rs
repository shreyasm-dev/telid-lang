use crate::error::{EvaluationError, EvaluationErrorKind};
use std::ops::Range;

pub fn error<T>(kind: EvaluationErrorKind, span: Range<usize>) -> Result<T, EvaluationError> {
  Err(EvaluationError { kind, span })
}
