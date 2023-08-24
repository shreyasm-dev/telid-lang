use super::ast::{Expression, ExpressionKind, Identifier, Statement, StatementKind};
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
    TokenKind::Void => ExpressionKind::Void,
    TokenKind::Identifier(identifier) => ExpressionKind::Identifier(Identifier(identifier)),
    TokenKind::NumberLiteral(number) => ExpressionKind::NumberLiteral(*number),
    TokenKind::StringLiteral(string) => ExpressionKind::StringLiteral(string),
    TokenKind::BooleanLiteral(boolean) => ExpressionKind::BooleanLiteral(boolean),
  }
  .map_with_span(|kind, span| Expression { kind, span });

  let statement = recursive(|statement| {
    let expression = recursive(|expression| {
      let binary_operator = choice((
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
      .map_with_span(|((operator, left), right), span| Expression {
        kind: ExpressionKind::Binary {
          operator: operator.to_binary_operator(),
          left: Box::new(left),
          right: Box::new(right),
        },
        span,
      });

      let unary_operator = choice((
        just(TokenKind::Plus),
        just(TokenKind::Minus),
        just(TokenKind::Bang),
      ))
      .then(expression.clone())
      .map_with_span(|(operator, operand), span| Expression {
        kind: ExpressionKind::Unary {
          operator: operator.to_unary_operator(),
          operand: Box::new(operand),
        },
        span,
      });

      let index = just(TokenKind::LeftBracket)
        .ignore_then(expression.clone())
        .then_ignore(just(TokenKind::RightBracket))
        .then(expression.clone())
        .map_with_span(|(index, iterable), span| Expression {
          kind: ExpressionKind::Index {
            index: Box::new(index),
            iterable: Box::new(iterable),
          },
          span,
        });

      let slice = just(TokenKind::LeftBracket)
        .ignore_then(expression.clone().or_not())
        .then_ignore(just(TokenKind::DotDot))
        .then(expression.clone().or_not())
        .then_ignore(just(TokenKind::RightBracket))
        .then(expression.clone())
        .map_with_span(|((start, end), iterable), span| Expression {
          kind: ExpressionKind::Slice {
            start: Box::new(start),
            end: Box::new(end),
            iterable: Box::new(iterable),
          },
          span,
        });

      let array_literal = delimited_list!(
        // Array literal
        expression,
        just(TokenKind::Comma),
        just(TokenKind::LeftBracket),
        just(TokenKind::RightBracket)
      )
      .map_with_span(|expressions, span| Expression {
        kind: ExpressionKind::ArrayLiteral(expressions),
        span,
      });

      let function_call = identifier
        .then(delimited_list!(
          expression,
          just(TokenKind::Comma),
          just(TokenKind::LeftParen),
          just(TokenKind::RightParen)
        ))
        .map_with_span(|(name, arguments), span| Expression {
          kind: ExpressionKind::FunctionCall { name, arguments },
          span,
        });

      let if_expression = just(TokenKind::If)
        .ignore_then(expression.clone())
        .then(statement.clone())
        .then(
          just(TokenKind::Else)
            .ignore_then(statement.clone())
            .or_not(),
        )
        .map_with_span(|((condition, consequence), alternative), span| Expression {
          kind: ExpressionKind::If {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
          },
          span,
        });

      let for_loop = just(TokenKind::For)
        .ignore_then(identifier)
        .then_ignore(just(TokenKind::In))
        .then(expression.clone())
        .then(statement.clone())
        .map_with_span(|((variable, iterable), body), span| Expression {
          kind: ExpressionKind::For {
            variable,
            iterable: Box::new(iterable),
            body: Box::new(body),
          },
          span,
        });

      let while_loop = just(TokenKind::While)
        .ignore_then(expression.clone())
        .then(statement.clone())
        .map_with_span(|(condition, body), span| Expression {
          kind: ExpressionKind::While {
            condition: Box::new(condition),
            body: Box::new(body),
          },
          span,
        });

      // Grouping
      expression
        .delimited_by(just(TokenKind::LeftParen), just(TokenKind::RightParen))
        .or(binary_operator)
        .or(unary_operator)
        .or(index)
        .or(slice)
        .or(array_literal)
        .or(function_call)
        .or(if_expression)
        .or(for_loop)
        .or(while_loop)
        .or(literal)
    });

    let assignment = identifier
      .then_ignore(just(TokenKind::Equals))
      .then(expression.clone())
      .map_with_span(|(name, value), span| Statement {
        kind: StatementKind::Assignment { name, value },
        span,
      });

    let let_declaration = just(TokenKind::Let)
    .ignore_then(identifier)
    .then_ignore(just(TokenKind::Equals))
    .then(expression.clone())
    .map_with_span(|(name, value), span| Statement {
      kind: StatementKind::Let {
        name,
        value,
        constant: false,
      },
      span,
    });

    let const_declaration = just(TokenKind::Let)
    .then_ignore(just(TokenKind::Const))
    .ignore_then(identifier)
    .then_ignore(just(TokenKind::Equals))
    .then(expression.clone())
    .map_with_span(|(name, value), span| Statement {
      kind: StatementKind::Let {
        name,
        value,
        constant: true,
      },
      span,
    });

    let function_declaration = just(TokenKind::Let)
    .then(just(TokenKind::Fn))
    .ignore_then(identifier)
    .then(identifier.repeated())
    .then_ignore(just(TokenKind::Equals))
    .then(statement.clone())
    .map_with_span(|((name, parameters), body), span| Statement {
      kind: StatementKind::FunctionDeclaration {
        name,
        parameters,
        body: Box::new(body),
      },
      span,
    });

    let expression_statement = expression.map_with_span(|expression, span| Statement {
      kind: StatementKind::Expression(expression),
      span,
    });

    statement
      .clone()
      .repeated()
      .delimited_by(just(TokenKind::LeftBrace), just(TokenKind::RightBrace))
      .map_with_span(|statements, span| Statement {
        kind: StatementKind::Block(statements),
        span,
      })
      .or(assignment)
      .or(let_declaration)
      .or(const_declaration)
      .or(function_declaration)
      .or(expression_statement)
      .then_ignore(just(TokenKind::Semicolon).or_not())
  });

  statement.repeated().then_ignore(just(TokenKind::Eof))
}
