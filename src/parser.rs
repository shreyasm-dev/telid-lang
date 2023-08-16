use crate::{
  ast::{Expression, Identifier, Statement},
  tokens::TokenKind,
};
use chumsky::{
  prelude::Simple,
  primitive::{choice, end, just},
  recursive::recursive,
  select, Parser,
};

macro_rules! delimited_list {
  ($element:ident, $separator:expr, $left:expr, $right:expr) => {
    $element
      .clone()
      .separated_by($separator)
      .delimited_by($left, $right)
  };
}

pub fn parser() -> impl Parser<TokenKind, Vec<Statement>, Error = Simple<TokenKind>> {
  // For when we don't want to wrap the identifier in an expression
  let plain_identifier = select! { TokenKind::Identifier(identifier) => Identifier(identifier) };

  let void = just(TokenKind::Void).map(|_| Expression::Void);
  let identifier =
    select! { TokenKind::Identifier(identifier) => Expression::Identifier(Identifier(identifier)) };
  let number_literal =
    select! { TokenKind::NumberLiteral(number) => Expression::NumberLiteral(*number) };
  let string_literal =
    select! { TokenKind::StringLiteral(string) => Expression::StringLiteral(string) };
  let boolean_literal =
    select! { TokenKind::BooleanLiteral(boolean) => Expression::BooleanLiteral(boolean) };

  let expression = recursive(|expression| {
    delimited_list!(
      // Array literal
      expression,
      just(TokenKind::Comma),
      just(TokenKind::LeftBracket),
      just(TokenKind::RightBracket)
    )
    .map(Expression::ArrayLiteral)
    .or(
      // Function call
      plain_identifier
        .then(delimited_list!(
          expression,
          just(TokenKind::Comma),
          just(TokenKind::LeftParen),
          just(TokenKind::RightParen)
        ))
        .map(|(name, parameters)| Expression::FunctionCall { name, parameters }),
    )
    .or(choice((
      void,
      identifier,
      number_literal,
      string_literal,
      boolean_literal,
    )))
    .or(
      // Grouping
      expression.delimited_by(just(TokenKind::LeftParen), just(TokenKind::RightParen)),
    )
  });

  let statement = recursive(|statement| {
    expression
      .clone()
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
      .map(Statement::ExpressionStatement)
      .or(
        // Block
        statement
          .repeated()
          .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace))
          .map(Statement::Block),
      )
  });

  statement.repeated().then(end()).map(|(output, _)| output)
}
