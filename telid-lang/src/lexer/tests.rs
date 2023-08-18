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
      (TokenKind::NumberLiteral(OrderedFloat(1.0)), 0..1),
      (TokenKind::Whitespace, 1..2),
      (TokenKind::Plus, 2..3),
      (TokenKind::Whitespace, 3..4),
      (TokenKind::NumberLiteral(OrderedFloat(2.25)), 4..8),
      (TokenKind::Eof, 4..8),
    ]
  );

  let source = "'This is a string\\nIt can handle newlines (without \\\\n)\nSlash\\'s string literals are cool'";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      (TokenKind::StringLiteral(
        "This is a string\nIt can handle newlines (without \\n)\nSlash's string literals are cool"
          .to_string()
      ), 0..90),
      (TokenKind::Eof, 0..90),
    ]
  );

  let source = "true false void";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(true);

  assert_eq!(
    tokens,
    vec![
      (TokenKind::BooleanLiteral(true), 0..4),
      (TokenKind::Whitespace, 4..5),
      (TokenKind::BooleanLiteral(false), 5..10),
      (TokenKind::Whitespace, 10..11),
      (TokenKind::Void, 11..15),
      (TokenKind::Eof, 11..15),
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
      (TokenKind::Let, 0..3),
      (TokenKind::Whitespace, 3..4),
      (TokenKind::Identifier("x".to_string()), 4..5),
      (TokenKind::Whitespace, 5..6),
      (TokenKind::Identifier("letx".to_string()), 6..10),
      (TokenKind::Whitespace, 10..11),
      (TokenKind::Identifier("xletx".to_string()), 11..16),
      (TokenKind::Whitespace, 16..17),
      (TokenKind::Identifier("xlet".to_string()), 17..21),
      (TokenKind::Eof, 17..21),
    ]
  );
}
