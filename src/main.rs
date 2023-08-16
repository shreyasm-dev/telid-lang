use crate::tokens::TokenKind;
use ariadne::Source;
use lexer::Lexer;
use parser::Parser;

mod ast;
mod error;
mod lexer;
mod parser;
mod tokens;

fn main() {
  let args = std::env::args().collect::<Vec<_>>();
  let path = args.get(1).expect("Expected path to source file").as_str();
  let source = std::fs::read_to_string(path).expect("Failed to read source file");

  let mut lexer = Lexer::new(source.as_str().clone());
  let tokens = lexer.lex();

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

  // remove comments, whitespace, and newlines
  let tokens = tokens
    .into_iter()
    .filter(|token| match token.kind {
      TokenKind::Comment | TokenKind::Whitespace | TokenKind::Newline => false,
      _ => true,
    })
    .collect::<Vec<_>>();

  let mut parser = Parser::new(tokens);
  let ast = parser.parse();

  match ast {
    Ok(ast) => println!("{:#?}", ast),
    Err(error) => {
      error
        .report(path)
        .print((path, Source::from(source.clone())))
        .unwrap();

      std::process::exit(1);
    }
  }
}
