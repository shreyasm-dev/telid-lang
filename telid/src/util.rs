use ariadne::{Label, Report, ReportKind};
use chumsky::{error::SimpleReason, prelude::Simple};
use std::ops::Range;
use telid_lang::lexer::tokens::TokenKind;

pub fn simple_error_to_string(error: Simple<TokenKind>) -> String {
  match error.reason() {
    SimpleReason::Unexpected => {
      format!(
        "Unexpected token: {:?}, expected one of: {:?}",
        match error.found() {
          Some(token) => token.as_ref(),
          None => "None",
        },
        error
          .expected()
          .map(|t| match t {
            Some(token) => token.as_ref(),
            None => "None",
          })
          .collect::<Vec<_>>()
      )
    }
    _ => format!("{:?}", error),
  }
}

pub fn simple_error_to_report<'a>(
  error: Simple<TokenKind>,
  id: &'a str,
  tokens: Vec<(TokenKind, Range<usize>)>,
) -> Report<'a, (&'a str, Range<usize>)> {
  Report::build(
    ReportKind::Error,
    id,
    tokens.get(error.span().start).unwrap().1.start,
  )
  .with_message(simple_error_to_string(error.clone()))
  .with_label(Label::new((
    id,
    tokens.get(error.span().start).unwrap().1.clone(),
  )))
  .finish()
}
