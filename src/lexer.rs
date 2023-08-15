use crate::{tokens::{Token, TokenKind}, error::LexError};
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

  pub fn lex(&mut self) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars = self.source.clone();
    let mut chars = chars.chars().peekable();

    while let Some(c) = chars.next() {
      tokens.push(self.lex_token(&mut chars, c));
    }

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

      // TODO: Implement shell literals (and shell commands)

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
      '.' => self.token(TokenKind::Dot),

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_literals() {
    let source = "1 + 2.25";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    assert_eq!(
      tokens,
      vec![
        Token {
          kind: TokenKind::NumberLiteral(1.0),
          span: 0..1
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 1..2
        },
        Token {
          kind: TokenKind::Plus,
          span: 2..3
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 3..4
        },
        Token {
          kind: TokenKind::NumberLiteral(2.25),
          span: 4..8
        },
      ]
    );

    let source = "'This is a string\\nIt can handle newlines (without \\\\n)\nSlash\\'s string literals are cool'";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    assert_eq!(
      tokens,
      vec![Token {
        kind: TokenKind::StringLiteral(
          "This is a string\nIt can handle newlines (without \\n)\nSlash's string literals are cool"
            .to_string()
        ),
        span: 0..90,
      }]
    );

    let source = "true false void";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    assert_eq!(
      tokens,
      vec![
        Token {
          kind: TokenKind::BooleanLiteral(true),
          span: 0..4
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 4..5
        },
        Token {
          kind: TokenKind::BooleanLiteral(false),
          span: 5..10
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 10..11
        },
        Token {
          kind: TokenKind::Void,
          span: 11..15
        },
      ]
    );
  }

  #[test]
  fn test_identifiers_keywords() {
    let source = "let x letx xletx xlet";
    let mut lexer = Lexer::new(source);
    let tokens = lexer.lex();

    assert_eq!(
      tokens,
      vec![
        Token {
          kind: TokenKind::Let,
          span: 0..3
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 3..4
        },
        Token {
          kind: TokenKind::Identifier("x".to_string()),
          span: 4..5
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 5..6
        },
        Token {
          kind: TokenKind::Identifier("letx".to_string()),
          span: 6..10
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 10..11
        },
        Token {
          kind: TokenKind::Identifier("xletx".to_string()),
          span: 11..16
        },
        Token {
          kind: TokenKind::Whitespace,
          span: 16..17
        },
        Token {
          kind: TokenKind::Identifier("xlet".to_string()),
          span: 17..21
        },
      ]
    );
  }
}
