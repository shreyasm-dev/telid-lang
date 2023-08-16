use crate::{
  ast::{Expression, Identifier, Statement},
  tokens::TokenKind,
};
use chumsky::{
  prelude::Simple,
  primitive::{choice, just},
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

pub fn parser() -> impl Parser<TokenKind, Statement, Error = Simple<TokenKind>> {
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

  let atom = recursive(|atom| {
    delimited_list!(
      // Array literal
      atom,
      just(TokenKind::Comma),
      just(TokenKind::LeftBracket),
      just(TokenKind::RightBracket)
    )
    .map(Expression::ArrayLiteral)
    .or(
      // Function call
      plain_identifier
        .then(delimited_list!(
          plain_identifier,
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
      atom.delimited_by(just(TokenKind::LeftParen), just(TokenKind::RightParen)),
    )
  });

  let expression = atom;

  let statement = expression.map(Statement::ExpressionStatement);

  statement
}
