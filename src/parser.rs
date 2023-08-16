use crate::{
  ast::{Expression, Identifier, Statement},
  tokens::TokenKind,
};
use chumsky::{prelude::Simple, primitive::just, recursive::recursive, select, Parser};

pub fn parser() -> impl Parser<TokenKind, Statement, Error = Simple<TokenKind>> {
  let void = just(TokenKind::Void).map(|_| Expression::Void);
  let identifier =
    select! { TokenKind::Identifier(identifier) => Expression::Identifier(Identifier(identifier)) };
  let number_literal =
    select! { TokenKind::NumberLiteral(number) => Expression::NumberLiteral(*number) };
  let string_literal =
    select! { TokenKind::StringLiteral(string) => Expression::StringLiteral(string) };
  let boolean_literal =
    select! { TokenKind::BooleanLiteral(boolean) => Expression::BooleanLiteral(boolean) };

  let atom = void
    .or(identifier)
    .or(number_literal)
    .or(string_literal)
    .or(boolean_literal)
    .or(recursive(|atom| {
      atom.delimited_by(just(TokenKind::LeftParen), just(TokenKind::RightParen))
    }));

  // TODO: Arrays cannot be nested
  let array = atom
    .clone()
    .separated_by(just(TokenKind::Comma))
    .delimited_by(just(TokenKind::LeftBracket), just(TokenKind::RightBracket))
    .map(Expression::ArrayLiteral);

  let expression = array.clone().or(atom);

  let statement = expression.map(Statement::ExpressionStatement);

  statement
}
