use crate::{evaluator::scope::Scope, lexer::tokens::TokenKind};
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser};
use evaluator::{evaluate, scope};
use lexer::Lexer;
use parser::parser;

mod error;
mod evaluator;
mod lexer;
mod parser;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let path = args.get(1).expect("Expected path to source file").as_str();

  run_file(path);
}

fn run_file(path: &str) {
  let source = std::fs::read_to_string(path).expect("Failed to read source file");
  run(&source, path, scope::default());
}

fn run(source: &str, id: &str, scope: Scope) {
  let mut lexer = Lexer::new(source.clone());
  let tokens = lexer.lex(false);

  for token in tokens.clone() {
    match token.kind {
      TokenKind::Error(error) => {
        error
          .report(id, token.span)
          .print((id, Source::from(source.clone())))
          .unwrap();

        std::process::exit(1);
      }
      _ => {}
    }
  }

  let parser = parser();
  let ast = parser.parse(
    tokens
      .clone()
      .into_iter()
      .map(|token| token.kind)
      .collect::<Vec<_>>(),
  );

  if let Err(error) = ast {
    for error in error {
      let message = match error.reason() {
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
      };

      Report::build(
        ReportKind::Error,
        id,
        tokens.get(error.span().start).unwrap().span.start,
      )
      .with_message(message)
      .with_label(Label::new((
        id,
        tokens.get(error.span().start).unwrap().span.clone(),
      )))
      .finish()
      .print((id, Source::from(source.clone())))
      .unwrap();
    }

    std::process::exit(1);
  }

  match evaluate(ast.unwrap(), scope) {
    Ok(_) => {}
    Err(error) => {
      println!("{}", error.to_string());
      std::process::exit(1);
    }
  }
}
