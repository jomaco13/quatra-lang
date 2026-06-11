use crate::qud::Qud;
use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    current: Token,
}

impl Parser {
    pub fn new(input: &str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut definitions = Vec::new();
        let mut main_expr = Expr::QudLiteral(Qud::Zero);

        while !matches!(self.current, Token::Eof) {
            match &self.current {
                Token::Func => definitions.push(self.parse_function()?),
                Token::Let => main_expr = self.parse_let()?,
                _ => main_expr = self.parse_expression()?,
            }
        }

        Ok(Program {
            functions: definitions,
            main: main_expr,
        })
    }

    pub fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }
    pub fn parse_top_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        loop {
            match self.current {
                Token::QPlus => {
                    self.bump();
                    let right = self.parse_and()?;
                    left = Expr::BinOp(Box::new(left), BinOperator::QAdd, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            match self.current {
                Token::QMul => {
                    self.bump();
                    let right = self.parse_unary()?;
                    left = Expr::BinOp(Box::new(left), BinOperator::QMul, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.current, Token::QNot) {
            self.bump();
            let operand = self.parse_unary()?;
            return Ok(Expr::UnaryOp(UnaryOperator::QNot, Box::new(operand)));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.current {
            Token::QudLit(value) => {
                let value = *value;
                self.bump();
                Ok(Expr::QudLiteral(value))
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.bump();
                if self.current == Token::LParen {
                    self.bump();
                    let mut args = Vec::new();
                    if self.current != Token::RParen {
                        loop {
                            args.push(self.parse_expression()?);
                            if self.current != Token::Comma {
                                break;
                            }
                            self.bump();
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::FuncCall(name, args))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            Token::Collapse => {
                self.bump();
                let inner = self.parse_primary()?;
                Ok(Expr::Collapse(Box::new(inner)))
            }
            Token::LParen => {
                self.bump();
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current)),
        }
    }

    fn parse_function(&mut self) -> Result<FuncDef, String> {
        self.expect(Token::Func)?;
        let name = self.expect_ident()?;
        self.expect(Token::LParen)?;
        let args = self.parse_comma_separated_identifiers()?;
        self.expect(Token::RParen)?;
        self.expect(Token::Equal)?;
        let body = self.parse_expression()?;

        Ok(FuncDef { name, args, body })
    }

    fn parse_let(&mut self) -> Result<Expr, String> {
        self.expect(Token::Let)?;
        let name = self.expect_ident()?;
        self.expect(Token::Equal)?;
        let value = self.parse_expression()?;
        self.expect(Token::In)?;
        let body = self.parse_expression()?;

        Ok(Expr::LetBind(name, Box::new(value), Box::new(body)))
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.current == expected {
            self.bump();
            Ok(())
        } else {
            Err(format!("Expected {:?}, found {:?}", expected, self.current))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match &self.current {
            Token::Ident(name) => {
                let name = name.clone();
                self.bump();
                Ok(name)
            }
            _ => Err(format!("Expected identifier, found {:?}", self.current)),
        }
    }

    fn parse_comma_separated_identifiers(&mut self) -> Result<Vec<String>, String> {
        if self.current == Token::RParen {
            return Ok(Vec::new());
        }
        let mut names = Vec::new();
        names.push(self.expect_ident()?);
        while self.current == Token::Comma {
            self.bump();
            if self.current == Token::RParen {
                break;
            }
            names.push(self.expect_ident()?);
        }
        Ok(names)
    }

    fn bump(&mut self) {
        self.current = self.lexer.next_token().unwrap_or(Token::Eof);
    }
}

#[cfg(test)]
mod tests {
    use crate::qud::Qud;
    use super::*;

    #[test]
    fn parses_literal() {
        let mut parser = Parser::new("0").unwrap();
        let program = parser.parse_program().unwrap();
        assert!(matches!(program.main, Expr::QudLiteral(Qud::Zero)));
    }

    #[test]
    fn parses_let() {
        let mut parser = Parser::new("let x = 2 in x").unwrap();
        let program = parser.parse_program().unwrap();
        assert!(matches!(program.main, Expr::LetBind(_, _, _)));
    }

    #[test]
    fn parses_function() {
        let mut parser = Parser::new("fun add(a, b) = a <+> b").unwrap();
        let program = parser.parse_program().unwrap();
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.functions[0].name, "add");
    }

    #[test]
    fn parses_collapse() {
        let mut parser = Parser::new("collapse 2").unwrap();
        let program = parser.parse_program().unwrap();
        assert!(matches!(program.main, Expr::Collapse(_)));
    }

    #[test]
    fn rejects_invalid_qud() {
        let result = Parser::new("4");
        assert!(result.is_err() || result.unwrap().parse_program().is_err());
    }
}
