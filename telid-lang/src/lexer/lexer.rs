use super::tokens::{Token, TokenKind};
use crate::error::LexError;
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
  source: &'a str,
  start: usize,
  current: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(source: &'a str) -> Lexer {
    Lexer {
      source,
      start: 0,
      current: 0,
    }
  }

  pub fn lex(&mut self, emit_ignored: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars = self.source.clone();
    let mut chars = chars.chars().peekable();

    while let Some(c) = chars.next() {
      let token = self.lex_token(&mut chars, c);
      match token.kind {
        TokenKind::Whitespace | TokenKind::Newline | TokenKind::Comment => {
          if emit_ignored {
            tokens.push(token);
          }
        }
        _ => tokens.push(token),
      }
    }

    tokens.push(self.token(TokenKind::Eof));

    tokens
  }

  pub fn lex_token(&mut self, chars: &mut Peekable<Chars<'_>>, c: char) -> Token {
    self.start = self.current;
    self.current += 1;

    match c {
      ' ' | '\t' => {
        while let Some(c) = chars.peek() {
          if !c.is_whitespace() {
            break;
          }

          chars.next();
          self.current += 1;
        }

        self.token(TokenKind::Whitespace)
      }
      '\n' => self.token(TokenKind::Newline), // TODO: Handle carriage returns

      '0'..='9' => {
        let mut literal = c.to_string();
        let mut has_dot = false;

        while let Some(d) = chars.peek() {
          if !d.is_digit(10) && *d != '.' {
            break;
          }

          if *d == '.' {
            if has_dot {
              break;
            } else {
              has_dot = true;
            }
          }

          literal.push(chars.next().unwrap());
          self.current += 1;
        }

        self.token(TokenKind::NumberLiteral(literal.parse().unwrap()))
      }

      '"' | '\'' => {
        let quote = c;
        let mut literal = String::new();
        let mut unterminated = self.is_at_end();

        while let Some(c) = chars.next() {
          self.current += 1;

          if c == quote {
            break;
          }

          if self.is_at_end() {
            unterminated = true;
            break;
          }

          if c == '\\' {
            if let Some(c) = chars.next() {
              self.current += 1;

              literal.push(match c {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '0' => '\0',
                _ => c,
              });
            }
          } else {
            literal.push(c);
          }
        }

        if unterminated {
          self.error(LexError::UnterminatedStringLiteral)
        } else {
          self.token(TokenKind::StringLiteral(literal))
        }
      }

      'a'..='z' | 'A'..='Z' | '_' => {
        let mut literal = c.to_string();

        while let Some(d) = chars.peek() {
          if !d.is_alphabetic() && !d.is_digit(10) && *d != '_' && *d != '$' {
            break;
          }

          literal.push(chars.next().unwrap());
          self.current += 1;
        }

        self.token(TokenKind::from_identifier(literal))
      }

      '+' => self.token(TokenKind::Plus),
      '-' => self.token(TokenKind::Minus),
      '*' => self.token(TokenKind::Asterisk),
      '/' => {
        if let Some('/') = chars.peek() {
          chars.next();
          self.current += 1;

          while let Some(c) = chars.next() {
            self.current += 1;

            if c == '\n' {
              break;
            }
          }

          self.token(TokenKind::Comment)
        } else if let Some('*') = chars.peek() {
          chars.next();
          self.current += 1;

          while let Some(c) = chars.next() {
            self.current += 1;

            if c == '*' {
              if let Some('/') = chars.peek() {
                chars.next();
                self.current += 1;
                break;
              }
            }
          }

          self.token(TokenKind::Comment)
        } else {
          self.token(TokenKind::Slash)
        }
      }
      '%' => self.token(TokenKind::Percent),
      '^' => self.token(TokenKind::Caret),
      '&' => {
        if let Some('&') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::AmpersandAmpersand)
        } else {
          self.token(TokenKind::Ampersand)
        }
      }
      '|' => {
        if let Some('|') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::PipePipe)
        } else {
          self.token(TokenKind::Pipe)
        }
      }
      '=' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::EqualsEquals)
        } else {
          self.token(TokenKind::Equals)
        }
      }
      '!' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::BangEquals)
        } else {
          self.token(TokenKind::Bang)
        }
      }
      '<' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::LessThanEquals)
        } else {
          self.token(TokenKind::LessThan)
        }
      }
      '>' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::GreaterThanEquals)
        } else {
          self.token(TokenKind::GreaterThan)
        }
      }

      '(' => self.token(TokenKind::LeftParen),
      ')' => self.token(TokenKind::RightParen),
      '[' => self.token(TokenKind::LeftBracket),
      ']' => self.token(TokenKind::RightBracket),
      '{' => self.token(TokenKind::LeftBrace),
      '}' => self.token(TokenKind::RightBrace),

      ',' => self.token(TokenKind::Comma),
      '.' => {
        if let Some('.') = chars.peek() {
          chars.next();
          self.current += 1;
          self.token(TokenKind::DotDot)
        } else {
          self.token(TokenKind::Dot)
        }
      }
      ';' => self.token(TokenKind::Semicolon),

      _ => self.error(LexError::UnexpectedCharacter(c)),
    }
  }

  pub fn token(&self, token: TokenKind) -> Token {
    Token {
      kind: token,
      span: self.start..self.current,
    }
  }

  pub fn error(&self, message: LexError) -> Token {
    Token {
      kind: TokenKind::Error(message),
      span: self.start..self.current,
    }
  }

  pub fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }
}
