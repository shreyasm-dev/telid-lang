use crate::{
  ast::{Expression, Identifier, Program, Statement},
  error::{ParseError, ParseErrorType},
  tokens::{Token, TokenKind},
};

pub struct Parser {
  tokens: Vec<Token>,
  position: usize,
}

impl Parser {
  // We need this because reasons (very good reasons, I hope)
  const NUMBER_LITERAL: TokenKind = TokenKind::NumberLiteral(0.0);
  const BOOLEAN_LITERAL: TokenKind = TokenKind::BooleanLiteral(false);
  const STRING_LITERAL: TokenKind = TokenKind::StringLiteral(String::new());
  const IDENTIFIER: TokenKind = TokenKind::Identifier(String::new());
  // const ERROR: TokenKind = TokenKind::Error(LexError::UnexpectedCharacter('\0'));

  pub fn new(tokens: Vec<Token>) -> Self {
    Self {
      tokens,
      position: 0,
    }
  }

  pub fn parse(&mut self) -> Result<Program, ParseError> {
    let mut statements = Vec::new();

    while !self.is_at_end() {
      let statement = self.parse_statement_or_expression(true)?;
      statements.push(statement);
    }

    Ok(Program(statements))
  }

  // TODO: Kind of a hack, let's not do this
  fn parse_statement_or_expression(&mut self, statements: bool) -> Result<Statement, ParseError> {
    let statement_tokens = vec![TokenKind::Let, TokenKind::LeftBrace];

    let expression_tokens = vec![
      TokenKind::LeftParen,
      TokenKind::Void,
      Self::IDENTIFIER,
      Self::NUMBER_LITERAL,
      Self::STRING_LITERAL,
      Self::BOOLEAN_LITERAL,
      TokenKind::LeftBracket,
      TokenKind::If,
    ];

    let expected = if statements {
      statement_tokens
        .iter()
        .chain(expression_tokens.iter())
        .cloned()
        .collect()
    } else {
      expression_tokens
    };

    match self.expect_one_of(expected)?.kind {
      TokenKind::LeftBrace => self.parse_block(),
      TokenKind::Let => self.parse_let(),

      TokenKind::LeftParen => {
        let expression = self.parse_statement_or_expression(false)?;
        self.expect(TokenKind::RightParen)?;
        Ok(expression)
      }
      TokenKind::Void => Ok(Statement::ExpressionStatement(Expression::Void)),
      TokenKind::Identifier(name) => Ok(Statement::ExpressionStatement(Expression::Identifier(
        Identifier(name),
      ))),
      TokenKind::NumberLiteral(value) => Ok(Statement::ExpressionStatement(
        Expression::NumberLiteral(value),
      )),
      TokenKind::StringLiteral(value) => Ok(Statement::ExpressionStatement(
        Expression::StringLiteral(value),
      )),
      TokenKind::BooleanLiteral(value) => Ok(Statement::ExpressionStatement(
        Expression::BooleanLiteral(value),
      )),
      TokenKind::LeftBracket => {
        let mut elements = Vec::new();

        while !self.is_at_end() && self.peek().kind != TokenKind::RightBracket {
          let element = self.parse_statement_or_expression(false)?;
          elements.push(element);

          match self
            .expect_one_of(vec![TokenKind::Comma, TokenKind::RightBracket])?
            .kind
          {
            TokenKind::Comma => {}
            TokenKind::RightBracket => break,
            _ => unreachable!(),
          }
        }

        Ok(Statement::ExpressionStatement(Expression::ArrayLiteral(
          elements,
        )))
      }
      TokenKind::If => {
        let condition = Box::new(self.parse_statement_or_expression(false)?);
        let consequence = Box::new(self.parse_statement_or_expression(true)?);
        let alternative = if self.peek().kind == TokenKind::Else {
          self.expect(TokenKind::Else)?;
          Some(Box::new(self.parse_statement_or_expression(true)?))
        } else {
          None
        };

        Ok(Statement::ExpressionStatement(Expression::If {
          condition,
          consequence,
          alternative,
        }))
      }

      _ => unreachable!(),
    }
  }

  fn parse_block(&mut self) -> Result<Statement, ParseError> {
    let mut statements = Vec::new();

    while !self.is_at_end() && self.peek().kind != TokenKind::RightBrace {
      let statement = self.parse_statement_or_expression(true)?;
      statements.push(statement);
    }

    self.expect(TokenKind::RightBrace)?;

    Ok(Statement::Block(statements))
  }

  fn parse_let(&mut self) -> Result<Statement, ParseError> {
    match self
      .expect_one_of(vec![Self::IDENTIFIER, TokenKind::Const, TokenKind::Fn])?
      .kind
    {
      TokenKind::Identifier(name) => {
        let name = Identifier(name);

        self.expect(TokenKind::Equals)?;
        let value = Box::new(self.parse_statement_or_expression(false)?);

        Ok(Statement::LetStatement {
          name,
          value,
          constant: false,
        })
      }
      TokenKind::Const => {
        let name = Identifier(match self.expect(Self::IDENTIFIER)?.kind {
          TokenKind::Identifier(name) => name,
          _ => unreachable!(),
        });

        self.expect(TokenKind::Equals)?;
        let value = Box::new(self.parse_statement_or_expression(false)?);

        Ok(Statement::LetStatement {
          name,
          value,
          constant: true,
        })
      }
      TokenKind::Fn => {
        // let fn name arg1 arg2 arg3 = <statement>;
        let name = Identifier(match self.expect(Self::IDENTIFIER)?.kind {
          TokenKind::Identifier(name) => name,
          _ => unreachable!(),
        });

        let mut parameters = Vec::new();

        while !self.is_at_end() && self.peek().kind != TokenKind::Equals {
          let parameter = match self.expect(Self::IDENTIFIER)?.kind {
            TokenKind::Identifier(name) => Identifier(name),
            _ => unreachable!(),
          };

          parameters.push(parameter);
        }

        self.expect(TokenKind::Equals)?;
        let body = self.parse_statement_or_expression(true)?;

        Ok(Statement::FunctionDeclaration {
          name,
          parameters,
          body: Box::new(body),
        })
      }
      _ => unreachable!(),
    }
  }

  fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
    if self.is_at_end() {
      return Err(ParseError::new(
        ParseErrorType::UnexpectedEnd(vec![kind]),
        self.position..self.position,
      ));
    }

    let token = self.tokens[self.position].clone();

    if !token.kind.clone().equals(kind.clone()) {
      return Err(ParseError::new(
        ParseErrorType::UnexpectedToken(token.kind.clone(), vec![kind]),
        token.span,
      ));
    }

    self.position += 1;

    Ok(token.clone())
  }

  fn expect_one_of(&mut self, kinds: Vec<TokenKind>) -> Result<Token, ParseError> {
    if self.is_at_end() {
      return Err(ParseError::new(
        ParseErrorType::UnexpectedEnd(kinds),
        self.position..self.position,
      ));
    }

    let token = self.tokens[self.position].clone();

    if !kinds
      .iter()
      .any(|kind| token.kind.clone().equals(kind.clone()))
    {
      return Err(ParseError::new(
        ParseErrorType::UnexpectedToken(token.kind.clone(), kinds),
        token.span,
      ));
    }

    self.position += 1;

    Ok(token.clone())
  }

  fn is_at_end(&self) -> bool {
    self.position >= self.tokens.len()
  }

  fn peek(&self) -> &Token {
    &self.tokens[self.position]
  }
}
