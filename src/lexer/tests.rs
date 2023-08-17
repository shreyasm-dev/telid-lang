use crate::lexer::{tokens::*, *};
use ordered_float::OrderedFloat;

#[test]
fn test_literals() {
  let source = "1 + 2.25";
  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      Token {
        kind: TokenKind::NumberLiteral(OrderedFloat(1.0)),
        span: 0..1
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 1..2
      },
      Token {
        kind: TokenKind::Plus,
        span: 2..3
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 3..4
      },
      Token {
        kind: TokenKind::NumberLiteral(OrderedFloat(2.25)),
        span: 4..8
      },
      Token {
        kind: TokenKind::Eof,
        span: 4..8
      },
    ]
  );

  let source = "'This is a string\\nIt can handle newlines (without \\\\n)\nSlash\\'s string literals are cool'";
  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      Token {
        kind: TokenKind::StringLiteral(
          "This is a string\nIt can handle newlines (without \\n)\nSlash's string literals are cool"
            .to_string()
        ),
        span: 0..90,
      },
      Token {
        kind: TokenKind::Eof,
        span: 0..90,
      },
    ]
  );

  let source = "true false void";
  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      Token {
        kind: TokenKind::BooleanLiteral(true),
        span: 0..4
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 4..5
      },
      Token {
        kind: TokenKind::BooleanLiteral(false),
        span: 5..10
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 10..11
      },
      Token {
        kind: TokenKind::Void,
        span: 11..15
      },
      Token {
        kind: TokenKind::Eof,
        span: 11..15
      },
    ]
  );
}

#[test]
fn test_identifiers_keywords() {
  let source = "let x letx xletx xlet";
  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      Token {
        kind: TokenKind::Let,
        span: 0..3
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 3..4
      },
      Token {
        kind: TokenKind::Identifier("x".to_string()),
        span: 4..5
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 5..6
      },
      Token {
        kind: TokenKind::Identifier("letx".to_string()),
        span: 6..10
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 10..11
      },
      Token {
        kind: TokenKind::Identifier("xletx".to_string()),
        span: 11..16
      },
      Token {
        kind: TokenKind::Whitespace,
        span: 16..17
      },
      Token {
        kind: TokenKind::Identifier("xlet".to_string()),
        span: 17..21
      },
      Token {
        kind: TokenKind::Eof,
        span: 17..21
      },
    ]
  );
}
