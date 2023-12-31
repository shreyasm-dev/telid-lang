use ariadne::Source;
use chumsky::Parser;
use inquire::{
  set_global_render_config,
  ui::{RenderConfig, StyleSheet, Styled},
  InquireError, Text,
};
use telid_lang::{
  evaluator::scope::Scope,
  evaluator::{evaluate, scope, value::Value},
  lexer::tokens::TokenKind,
  lexer::Lexer,
  parser::parser,
};
use util::simple_error_to_report;

mod util;

fn main() {
  set_global_render_config(get_repl_render_config());

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

fn get_repl_render_config() -> RenderConfig {
  let prefix = Styled::new(">");

  let mut render_config = RenderConfig::default();
  render_config.prompt_prefix = prefix.clone();
  render_config.answer = StyleSheet::new();
  render_config.answered_prompt_prefix = prefix.clone();

  render_config
}

fn run_file(path: &str) -> Result<(Value, Scope), ()> {
  let source = std::fs::read_to_string(path).expect("Failed to read source file");
  run(&source, path, scope::default())
}

fn run_repl() {
  let mut scope = scope::default();

  loop {
    let input = Text::new("").prompt();

    match input {
      Ok(input) => {
        if let Ok((output, scope_)) = run(&input, "repl", scope.clone()) {
          if output != Value::Void {
            println!("{}", output.to_string());
          }

          scope = scope_;
        }
      }
      Err(error) => match error {
        InquireError::OperationCanceled | InquireError::OperationInterrupted => {
          println!("Type exit(0) to exit")
        }
        _ => panic!("Unexpected error: {:?}", error),
      },
    }
  }
}

fn run(source: &str, id: &str, scope: Scope) -> Result<(Value, Scope), ()> {
  let mut lexer = Lexer::new(source.clone());
  let tokens = lexer.lex(false);

  for token in tokens.clone() {
    match token.0 {
      TokenKind::Error(error) => {
        error
          .report(id, token.1)
          .eprint((id, Source::from(source.clone())))
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
      .map(|token| token.0)
      .collect::<Vec<_>>(),
  );

  if let Err(error) = ast {
    for error in error {
      simple_error_to_report(error, id, tokens.clone())
        .eprint((id, Source::from(source.clone())))
        .unwrap();
    }

    return Err(());
  }

  match evaluate(ast.unwrap(), scope.clone()) {
    Ok(scope) => Ok(scope),
    Err(error) => {
      error
        .report(id, error.span.clone(), tokens)
        .eprint((id, Source::from(source.clone())))
        .unwrap();
      Err(())
    }
  }
}
