use crate::{
  ast::{Expression, Identifier, Statement, UnaryOperator},
  tokens::TokenKind,
};
use chumsky::{
  prelude::Simple,
  primitive::{choice, just},
  recursive::recursive,
  select, Parser,
};

macro_rules! delimited_list {
  ($element:expr, $separator:expr, $left:expr, $right:expr) => {
    $element
      .clone()
      .separated_by($separator)
      .delimited_by($left, $right)
  };
}

pub fn parser() -> impl Parser<TokenKind, Vec<Statement>, Error = Simple<TokenKind>> {
  // For when we don't want to wrap the identifier in an expression
  let identifier = select! { TokenKind::Identifier(identifier) => Identifier(identifier) };
  let literal = select! {
    TokenKind::Void => Expression::Void,
    TokenKind::Identifier(identifier) => Expression::Identifier(Identifier(identifier)),
    TokenKind::NumberLiteral(number) => Expression::NumberLiteral(*number),
    TokenKind::StringLiteral(string) => Expression::StringLiteral(string),
    TokenKind::BooleanLiteral(boolean) => Expression::BooleanLiteral(boolean),
  };

  let statement = recursive(|statement| {
    let expression = recursive(|expression| {
      // Grouping
      expression
        .clone()
        .delimited_by(just(TokenKind::LeftParen), just(TokenKind::RightParen))
        .or(
          // Unary operator
          choice((
            just(TokenKind::Plus),
            just(TokenKind::Minus),
            just(TokenKind::Bang),
          ))
          .then(expression.clone())
          .map(|(operator, operand)| Expression::Unary {
            operator: match operator {
              TokenKind::Plus => UnaryOperator::Identity,
              TokenKind::Minus => UnaryOperator::Negate,
              TokenKind::Bang => UnaryOperator::Not,
              _ => unreachable!(),
            },
            operand: Box::new(operand),
          }),
        )
        .or(
          delimited_list!(
            // Array literal
            expression,
            just(TokenKind::Comma),
            just(TokenKind::LeftBracket),
            just(TokenKind::RightBracket)
          )
          .map(Expression::ArrayLiteral),
        )
        .or(
          // Function call
          identifier
            .then(delimited_list!(
              expression,
              just(TokenKind::Comma),
              just(TokenKind::LeftParen),
              just(TokenKind::RightParen)
            ))
            .map(|(name, parameters)| Expression::FunctionCall { name, parameters }),
        )
        .or(
          // If expression
          // TODO: Is there a way to do this without using separate parsers for if and if-else?
          just(TokenKind::If)
            .then(expression.clone())
            .then(statement.clone())
            .then(just(TokenKind::Else))
            .then(statement.clone())
            .map(
              |((((_, condition), consequence), _), alternative)| Expression::If {
                condition: Box::new(condition),
                consequence: Box::new(consequence),
                alternative: Box::new(Some(alternative)),
              },
            )
            .or(
              just(TokenKind::If)
                .then(expression.clone())
                .then(statement.clone())
                .map(|((_, condition), consequence)| Expression::If {
                  condition: Box::new(condition),
                  consequence: Box::new(consequence),
                  alternative: Box::new(None),
                }),
            ),
        )
        .or(
          // For loop
          just(TokenKind::For)
            .then(identifier)
            .then(just(TokenKind::In))
            .then(expression)
            .then(statement.clone())
            .map(|((((_, variable), _), iterable), body)| Expression::For {
              variable,
              iterable: Box::new(iterable),
              body: Box::new(body),
            }),
        )
        .or(literal)
    });

    expression.clone().map(Statement::ExpressionStatement).or(
      // Block
      statement
        .clone()
        .repeated()
        .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace))
        .map(Statement::Block)
        .or(
          // Let declaration
          just(TokenKind::Let)
            .then(identifier)
            .then(just(TokenKind::Equals))
            .then(expression.clone())
            .map(|(((_, name), _), value)| Statement::LetStatement {
              name,
              value,
              constant: false,
            }),
        )
        .or(
          // Const declaration
          just(TokenKind::Let)
            .then(just(TokenKind::Const))
            .then(identifier)
            .then(just(TokenKind::Equals))
            .then(expression)
            .map(|((((_, _), name), _), value)| Statement::LetStatement {
              name,
              value,
              constant: true,
            }),
        )
        .or(
          // Function declaration
          just(TokenKind::Let)
            .then(just(TokenKind::Fn))
            .then(identifier)
            .then(identifier.repeated())
            .then(just(TokenKind::Equals))
            .then(statement.clone())
            .map(
              |((((_, name), parameters), _), body)| Statement::FunctionDeclaration {
                name,
                parameters,
                body: Box::new(body),
              },
            ),
        ),
    )
  });

  statement
    .repeated()
    .then(just(TokenKind::Eof))
    .map(|(output, _)| output)
}
