use crate::{
  evaluator::{Scope, Value, Variable},
  tokens::TokenKind,
};
use ariadne::{Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser};
use evaluator::evaluate;
use lexer::Lexer;
use parser::parser;

mod ast;
mod error;
mod evaluator;
mod lexer;
mod parser;
mod tokens;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let path = args.get(1).expect("Expected path to source file").as_str();
  let source = std::fs::read_to_string(path).expect("Failed to read source file");

  let mut lexer = Lexer::new(source.as_str().clone());
  let tokens = lexer.lex(false);

  for token in tokens.clone() {
    match token.kind {
      TokenKind::Error(error) => {
        error
          .report(path, token.span)
          .print((path, Source::from(source.clone())))
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
        path,
        tokens.get(error.span().start).unwrap().span.start,
      )
      .with_message(message)
      .with_label(Label::new((
        path,
        tokens.get(error.span().start).unwrap().span.clone(),
      )))
      .finish()
      .print((path, Source::from(source.clone())))
      .unwrap();
    }

    std::process::exit(1);
  }

  let mut scope = Scope::new();

  scope.insert(
    String::from("println"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |parameters| {
          println!("{}", parameters[0].to_string());
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  scope.insert(
    String::from("print"),
    Variable {
      value: Value::RustFunction {
        parameter_count: 1,
        function: |parameters| {
          print!("{}", parameters[0].to_string());
          Ok(Value::Void)
        },
      },
      constant: true,
    },
  );

  match evaluate(ast.unwrap(), scope) {
    Ok(_) => {}
    Err(error) => {
      println!("{}", error.to_string());
      std::process::exit(1);
    }
  }
}
