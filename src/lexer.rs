#![allow(dead_code)]
use std::{fs, io};

use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub struct Lexer {
    input: std::iter::Peekable<std::vec::IntoIter<char>>,
    line: u32,
}

#[derive(Debug)]
pub enum ParserError {
    IoError(std::io::Error),
    Other,
}

impl std::error::Error for ParserError {}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "Hello {:?}", e),
            _ => {
                todo!()
            }
        }
    }
}

impl From<io::Error> for ParserError {
    fn from(value: io::Error) -> Self {
        ParserError::IoError(value)
    }
}

impl Lexer {
    pub fn from_file(file_name: impl ToString) -> Result<Self, ParserError> {
        let input = fs::read_to_string(file_name.to_string())?;
        Ok(Self::new(input))
    }

    pub fn new(input: impl Into<String>) -> Self {
        let owned_string: String = input.into();
        let chars_vec: Vec<char> = owned_string.chars().collect();

        Lexer {
            input: chars_vec.into_iter().peekable(),
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.input.next()?;
        if ch == '\n' {
            self.line += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let ch = self.advance()?;
        let start_line = self.line;
        let token_kind = match ch {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '/' => TokenKind::Slash,
            '*' => TokenKind::Star,
            '>' => {
                if let Some(_) = self.input.next_if_eq(&'=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            '<' => {
                if let Some(_) = self.input.next_if_eq(&'=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '!' => {
                if let Some(_) = self.input.next_if_eq(&'=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,

            '=' => {
                if let Some(_) = self.input.next_if_eq(&'=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '"' => {
                // String or not
                todo!()
            }
            'a'..='z' | 'A'..='Z' | '_' => TokenKind::Identifier,
            c if is_khmer_digit(c) => {
                todo!()
            }
            c if is_khmer_char(c) => {
                let mut literal: String = c.to_string();
                while let Some(&next) = self.input.peek() {
                    if !next.is_whitespace() || next.is_ascii_digit() || next != '_' {
                        if next.is_ascii_punctuation() && next != '_' {
                            break;
                        }
                        literal.push(next);
                        self.advance();
                    } else {
                        break;
                    }
                }

                let token_kind = match literal.as_str() {
                    "តាង" => TokenKind::Var,
                    "បើ" => TokenKind::If,
                    "បើពុំនោះទេ" => TokenKind::Else,
                    "បោះពុម្ព" => TokenKind::Print,
                    "អនុគមន៍" => TokenKind::Fun,
                    "ពុម្ពគំរូ" => TokenKind::Class,
                    "គ្មានតម្លៃ" => TokenKind::Null,
                    "ឬ" => TokenKind::Or,
                    "និង" => TokenKind::And,
                    "ពិត" => TokenKind::True,
                    "មិនពិត" => TokenKind::False,
                    _ => {
                        todo!()
                    }
                };
                token_kind
            }
            _ => todo!(),
        };

        Some(Token {
            kind: token_kind,
            line: start_line,
            literal: None,
        })
    }

    fn next_line(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }
}

fn is_khmer_char(c: char) -> bool {
    matches!(c, '\u{1780}'..='\u{17FF}')
}

fn is_khmer_digit(c: char) -> bool {
    matches!(c, '\u{17E0}'..='\u{17E9}')
}

fn khmer_digit_to_int(c: char) -> Option<u32> {
    if is_khmer_digit(c) {
        Some((c as u32) - 0x17E0)
    } else {
        None
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenKind};

    #[test]
    fn keyword() {
        let buffer = "តាង";
        let mut lexer = Lexer::new(buffer);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.kind, TokenKind::Var);
    }
}
