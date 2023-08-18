use crate::{
  lexer::Lexer,
  parser::ast::{BinaryOperator, Statement, StatementKind},
  parser::{
    ast::{Expression, ExpressionKind, UnaryOperator},
    parser,
  },
};
use chumsky::Parser;

#[test]
fn test_operators() {
  let source = "- - 5 6";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(false);
  let tokens = tokens.iter().map(|t| t.0.clone()).collect::<Vec<_>>();

  let ast = parser().parse(tokens);

  assert_eq!(
    ast,
    Ok(vec![Statement {
      kind: StatementKind::Expression(Expression {
        kind: ExpressionKind::Unary {
          operator: UnaryOperator::Negate,
          operand: Box::new(Expression {
            kind: ExpressionKind::Binary {
              operator: BinaryOperator::Subtract,
              left: Box::new(Expression {
                kind: ExpressionKind::NumberLiteral(5.0),
                span: 2..3,
              }),
              right: Box::new(Expression {
                kind: ExpressionKind::NumberLiteral(6.0),
                span: 3..4,
              }),
            },
            span: 1..4,
          }),
        },
        span: 0..4,
      }),
      span: 0..4,
    }])
  );

  let source = "- (-5) 6";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(false);
  let tokens = tokens.iter().map(|t| t.0.clone()).collect::<Vec<_>>();

  let ast = parser().parse(tokens);

  assert_eq!(
    ast,
    Ok(vec![Statement {
      kind: StatementKind::Expression(Expression {
        kind: ExpressionKind::Binary {
          operator: BinaryOperator::Subtract,
          left: Box::new(Expression {
            kind: ExpressionKind::Unary {
              operator: UnaryOperator::Negate,
              operand: Box::new(Expression {
                kind: ExpressionKind::NumberLiteral(5.0),
                span: 3..4,
              }),
            },
            span: 2..4,
          }),
          right: Box::new(Expression {
            kind: ExpressionKind::NumberLiteral(6.0),
            span: 5..6,
          }),
        },
        span: 0..6,
      }),
      span: 0..6,
    }])
  );
}
