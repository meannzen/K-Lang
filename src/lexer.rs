#![allow(dead_code)]
use std::{fs, io};

use crate::token::{Token, TokenType};

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
        let token = match ch {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '>' => {
                if let Some(next_char) = self.input.peek()
                    && *next_char == '='
                {
                    self.advance();
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '<' => {
                if let Some(next_char) = self.input.peek()
                    && *next_char == '='
                {
                    self.advance();
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '!' => {
                if let Some(next_char) = self.input.peek()
                    && *next_char == '='
                {
                    self.advance();
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }

            '=' => {
                if let Some(next_char) = self.input.peek()
                    && *next_char == '='
                {
                    self.advance();
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => TokenType::Identifier,
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

                let token_type = match literal.as_str() {
                    "តាង" => TokenType::Var,
                    "បើ" => TokenType::If,
                    "បើពុំនោះទេ" => TokenType::Else,
                    "បោះពុម្ព" => TokenType::Print,
                    "អនុគមន៍" => TokenType::Fun,
                    "ពុម្ពគំរូ" => TokenType::Class,
                    "គ្មានតម្លៃ" => TokenType::Null,
                    "ឬ" => TokenType::Or,
                    "និង" => TokenType::And,
                    "ពិត" => TokenType::True,
                    "មិនពិត" => TokenType::False,
                    _ => {
                        todo!()
                    }
                };
                token_type
            }
            _ => todo!(),
        };

        Some(Token {
            token_type: token,
            line: start_line,
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

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Lexer, token::TokenType};

    #[test]
    fn empty_string() {
        let buffer = "តាង";
        let mut lexer = Lexer::new(buffer);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token_type, TokenType::Identifier);
    }
}
