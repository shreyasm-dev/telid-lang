use super::ast::{Expression, Identifier, Statement, UnaryOperator};
use crate::lexer::tokens::TokenKind;
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
          // Binary operator
          choice((
            just(TokenKind::Plus),
            just(TokenKind::Minus),
            just(TokenKind::Asterisk),
            just(TokenKind::Slash),
            just(TokenKind::Percent),
            just(TokenKind::EqualsEquals),
            just(TokenKind::BangEquals),
            just(TokenKind::LessThan),
            just(TokenKind::LessThanEquals),
            just(TokenKind::GreaterThan),
            just(TokenKind::GreaterThanEquals),
            just(TokenKind::AmpersandAmpersand),
            just(TokenKind::PipePipe),
            just(TokenKind::DotDot),
          ))
          .then(expression.clone())
          .then(expression.clone())
          .map(|((operator, left), right)| Expression::Binary {
            operator: operator.to_binary_operator(),
            left: Box::new(left),
            right: Box::new(right),
          }),
        )
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
          // Index
          just(TokenKind::LeftBracket)
            .ignore_then(expression.clone())
            .then_ignore(just(TokenKind::RightBracket))
            .then(expression.clone())
            .map(|(index, iterable)| Expression::Index {
              iterable: Box::new(iterable),
              index: Box::new(index),
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
            .map(|(name, arguments)| Expression::FunctionCall { name, arguments }),
        )
        .or(
          // If expression
          just(TokenKind::If)
            .ignore_then(expression.clone())
            .then(statement.clone())
            .then(
              just(TokenKind::Else)
                .ignore_then(statement.clone())
                .or_not(),
            )
            .map(|((condition, consequence), alternative)| Expression::If {
              condition: Box::new(condition),
              consequence: Box::new(consequence),
              alternative: Box::new(alternative),
            }),
        )
        .or(
          // For loop
          just(TokenKind::For)
            .ignore_then(identifier)
            .then_ignore(just(TokenKind::In))
            .then(expression.clone())
            .then(statement.clone())
            .map(|((variable, iterable), body)| Expression::For {
              variable,
              iterable: Box::new(iterable),
              body: Box::new(body),
            }),
        )
        .or(
          // While loop
          just(TokenKind::While)
            .ignore_then(expression)
            .then(statement.clone())
            .map(|(condition, body)| Expression::While {
              condition: Box::new(condition),
              body: Box::new(body),
            }),
        )
        .or(literal)
    });

    statement
      .clone()
      .repeated()
      .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace))
      .map(Statement::Block)
      .or(
        // Assignment
        identifier
          .then_ignore(just(TokenKind::Equals))
          .then(expression.clone())
          .map(|(name, value)| Statement::Assignment { name, value }),
      )
      .or(
        // Let declaration
        just(TokenKind::Let)
          .ignore_then(identifier)
          .then_ignore(just(TokenKind::Equals))
          .then(expression.clone())
          .map(|(name, value)| Statement::LetStatement {
            name,
            value,
            constant: false,
          }),
      )
      .or(
        // Const declaration
        just(TokenKind::Let)
          .then_ignore(just(TokenKind::Const))
          .ignore_then(identifier)
          .then_ignore(just(TokenKind::Equals))
          .then(expression.clone())
          .map(|(name, value)| Statement::LetStatement {
            name,
            value,
            constant: true,
          }),
      )
      .or(
        // Function declaration
        just(TokenKind::Let)
          .then(just(TokenKind::Fn))
          .ignore_then(identifier)
          .then(identifier.repeated())
          .then_ignore(just(TokenKind::Equals))
          .then(statement.clone())
          .map(
            |((name, parameters), body)| Statement::FunctionDeclaration {
              name,
              parameters,
              body: Box::new(body),
            },
          ),
      )
      .or(
        // Expression statement
        expression.map(Statement::ExpressionStatement),
      )
      .then_ignore(just(TokenKind::Semicolon).or_not())
  });

  statement.repeated().then_ignore(just(TokenKind::Eof))
}
