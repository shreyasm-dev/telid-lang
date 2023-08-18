use crate::{
  lexer::Lexer,
  parser::ast::{BinaryOperator, Statement},
  parser::{
    ast::{Expression, UnaryOperator},
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
    Ok(vec![Statement::ExpressionStatement(Expression::Unary {
      operator: UnaryOperator::Negate,
      operand: Box::new(Expression::Binary {
        left: Box::new(Expression::NumberLiteral(5.0)),
        operator: BinaryOperator::Subtract,
        right: Box::new(Expression::NumberLiteral(6.0)),
      }),
    })])
  );

  let source = "- (-5) 6";

  let mut lexer = Lexer::new(source);
  let tokens = lexer.lex(false);
  let tokens = tokens.iter().map(|t| t.0.clone()).collect::<Vec<_>>();

  let ast = parser().parse(tokens);

  assert_eq!(
    ast,
    Ok(vec![Statement::ExpressionStatement(Expression::Binary {
      left: Box::new(Expression::Unary {
        operator: UnaryOperator::Negate,
        operand: Box::new(Expression::NumberLiteral(5.0)),
      }),
      operator: BinaryOperator::Subtract,
      right: Box::new(Expression::NumberLiteral(6.0)),
    })])
  );
}
