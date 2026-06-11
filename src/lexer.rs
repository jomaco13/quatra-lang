use crate::qud::Qud;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    QudLit(Qud),
    Ident(String),
    Func,
    Type,
    Let,
    In,
    Collapse,
    QPlus,
    QMul,
    QNot,
    Arrow,
    Equal,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Comma,
    StringLit(String),
    Comment,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments();
        if self.pos >= self.input.len() {
            return Ok(Token::Eof);
        }

        let ch = self.current_char();

        if ch.is_ascii_digit() {
            let val = ch.to_digit(10).unwrap() as u8;
            self.advance();
            if val > 3 {
                return Err(format!("Invalid Qud literal: {} (must be 0-3)", val));
            }
            return Ok(Token::QudLit(Qud::from_u8(val).unwrap()));
        }

        if ch == '"' {
            return self.read_string();
        }

        if ch.is_alphabetic() || ch == '_' {
            let mut ident = String::new();
            while self.pos < self.input.len()
                && (self.current_char().is_alphanumeric() || self.current_char() == '_')
            {
                ident.push(self.current_char());
                self.advance();
            }
            return Ok(self.match_keyword(ident));
        }

        if ch == '<' && self.peek() == '+' && self.input.get(self.pos + 2) == Some(&'>') {
            self.advance();
            self.advance();
            self.advance();
            return Ok(Token::QPlus);
        }
        if ch == '<' && self.peek() == '*' && self.input.get(self.pos + 2) == Some(&'>') {
            self.advance();
            self.advance();
            self.advance();
            return Ok(Token::QMul);
        }
        if ch == '-' && self.peek() == '>' {
            self.advance();
            self.advance();
            return Ok(Token::Arrow);
        }

        if ch == '~' && self.peek() == '~' {
            self.advance();
            self.advance();
            return Ok(Token::QNot);
        }

        self.advance();
        match ch {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '=' => Ok(Token::Equal),
            _ => Err(format!("Unknown character: {}", ch)),
        }
    }

    fn current_char(&self) -> char {
        self.input[self.pos]
    }

    fn peek(&self) -> char {
        if self.pos + 1 < self.input.len() {
            self.input[self.pos + 1]
        } else {
            '\0'
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn skip_whitespace_and_comments(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.advance();
                continue;
            }
            if ch == '-' && self.peek() == '-' {
                while self.pos < self.input.len() && self.current_char() != '\n' {
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        self.advance(); // skip opening quote
        let mut value = String::new();
        while self.pos < self.input.len() && self.current_char() != '"' {
            if self.current_char() == '\\' {
                self.advance();
                if self.pos < self.input.len() {
                    value.push(self.current_char());
                    self.advance();
                }
            } else {
                value.push(self.current_char());
                self.advance();
            }
        }
        if self.pos >= self.input.len() {
            return Err("Unterminated string literal".to_string());
        }
        self.advance(); // skip closing quote
        Ok(Token::StringLit(value))
    }

    fn match_keyword(&self, ident: String) -> Token {
        match ident.as_str() {
            "fun" => Token::Func,
            "type" => Token::Type,
            "let" => Token::Let,
            "in" => Token::In,
            "collapse" => Token::Collapse,
            _ => Token::Ident(ident),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qud_literals() {
        let mut lexer = Lexer::new("0 1 2 3");
        assert_eq!(lexer.next_token().unwrap(), Token::QudLit(Qud::Zero));
        assert_eq!(lexer.next_token().unwrap(), Token::QudLit(Qud::One));
        assert_eq!(lexer.next_token().unwrap(), Token::QudLit(Qud::Super));
        assert_eq!(lexer.next_token().unwrap(), Token::QudLit(Qud::Error));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new("<+> <*> ~~ ->");
        assert_eq!(lexer.next_token(), Ok(Token::QPlus));
        assert_eq!(lexer.next_token(), Ok(Token::QMul));
        assert_eq!(lexer.next_token(), Ok(Token::QNot));
        assert_eq!(lexer.next_token(), Ok(Token::Arrow));
    }

    #[test]
    fn test_invalid_qud() {
        let mut lexer = Lexer::new("4");
        assert!(lexer.next_token().is_err());
    }
}
