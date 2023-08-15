use crate::tokens::TokenKind;
use ariadne::Source;
use lexer::Lexer;

mod ast;
mod error;
mod lexer;
mod tokens;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let path = args.get(1).expect("Expected path to source file").as_str();
  let source = std::fs::read_to_string(path).expect("Failed to read source file");

  let mut lexer = Lexer::new(source.as_str().clone());
  let tokens = lexer.lex();

  for token in tokens {
    match token.kind {
      TokenKind::Error(error) => {
        error
          .report(path, token.span)
          .print((path, Source::from(source.clone())))
          .unwrap();
      }
      _ => {}
    }
  }
}
