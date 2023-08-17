use crate::{evaluator::scope::Scope, lexer::tokens::TokenKind};
use ariadne::Source;
use chumsky::Parser;
use evaluator::{evaluate, scope};
use lexer::Lexer;
use parser::parser;
use std::io::{stdin, stdout, Write};
use util::simple_error_to_report;

mod error;
mod evaluator;
mod lexer;
mod parser;
mod util;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  match args.get(1) {
    Some(path) => {
      if let Err(_) = run_file(path) {
        std::process::exit(1);
      }
    }
    None => run_repl(),
  }
}

fn run_file(path: &str) -> Result<Scope, ()> {
  let source = std::fs::read_to_string(path).expect("Failed to read source file");
  run(&source, path, scope::default())
}

fn run_repl() {
  let mut scope = scope::default();

  loop {
    print!("> ");
    stdout().flush().unwrap();
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");

    if let Ok(scope_) = run(&input, "repl", scope.clone()) {
      scope = scope_;
    }
  }
}

fn run(source: &str, id: &str, scope: Scope) -> Result<Scope, ()> {
  let mut lexer = Lexer::new(source.clone());
  let tokens = lexer.lex(false);

  for token in tokens.clone() {
    match token.kind {
      TokenKind::Error(error) => {
        error
          .report(id, token.span)
          .print((id, Source::from(source.clone())))
          .unwrap();

        return Err(());
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
      simple_error_to_report(error, id, tokens.clone())
        .print((id, Source::from(source.clone())))
        .unwrap();
    }

    return Err(());
  }

  match evaluate(ast.unwrap(), scope.clone()) {
    Ok(scope) => Ok(scope),
    Err(error) => {
      println!("{}", error.to_string());
      Err(())
    }
  }
}
