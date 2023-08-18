use super::tokens::TokenKind;
use crate::error::LexError;
use std::{iter::Peekable, ops::Range, str::Chars};

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

  pub fn lex(&mut self, emit_ignored: bool) -> Vec<(TokenKind, Range<usize>)> {
    let mut tokens = Vec::new();
    let chars = self.source.clone();
    let mut chars = chars.chars().peekable();

    while let Some(c) = chars.next() {
      let kind = self.lex_token(&mut chars, c);
      match kind {
        TokenKind::Whitespace | TokenKind::Newline | TokenKind::Comment => {
          if emit_ignored {
            tokens.push(self.token(kind));
          }
        }
        _ => tokens.push(self.token(kind)),
      }
    }

    tokens.push(self.token(TokenKind::Eof));

    tokens
  }

  pub fn lex_token(&mut self, chars: &mut Peekable<Chars<'_>>, c: char) -> TokenKind {
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

        TokenKind::Whitespace
      }
      '\n' => TokenKind::Newline, // TODO: Handle carriage returns

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

        TokenKind::NumberLiteral(literal.parse().unwrap())
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
          TokenKind::Error(LexError::UnterminatedStringLiteral)
        } else {
          TokenKind::StringLiteral(literal)
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

        TokenKind::from_identifier(literal)
      }

      '+' => TokenKind::Plus,
      '-' => TokenKind::Minus,
      '*' => TokenKind::Asterisk,
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

          TokenKind::Comment
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

          TokenKind::Comment
        } else {
          TokenKind::Slash
        }
      }
      '%' => TokenKind::Percent,
      '^' => TokenKind::Caret,
      '&' => {
        if let Some('&') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::AmpersandAmpersand
        } else {
          TokenKind::Ampersand
        }
      }
      '|' => {
        if let Some('|') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::PipePipe
        } else {
          TokenKind::Pipe
        }
      }
      '=' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::EqualsEquals
        } else {
          TokenKind::Equals
        }
      }
      '!' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::BangEquals
        } else {
          TokenKind::Bang
        }
      }
      '<' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::LessThanEquals
        } else {
          TokenKind::LessThan
        }
      }
      '>' => {
        if let Some('=') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::GreaterThanEquals
        } else {
          TokenKind::GreaterThan
        }
      }

      '(' => TokenKind::LeftParen,
      ')' => TokenKind::RightParen,
      '[' => TokenKind::LeftBracket,
      ']' => TokenKind::RightBracket,
      '{' => TokenKind::LeftBrace,
      '}' => TokenKind::RightBrace,

      ',' => TokenKind::Comma,
      '.' => {
        if let Some('.') = chars.peek() {
          chars.next();
          self.current += 1;
          TokenKind::DotDot
        } else {
          TokenKind::Dot
        }
      }
      ';' => TokenKind::Semicolon,

      _ => TokenKind::Error(LexError::UnexpectedCharacter(c)),
    }
  }

  pub fn token(&self, kind: TokenKind) -> (TokenKind, Range<usize>) {
    (kind, self.start..self.current)
  }

  pub fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }
}
