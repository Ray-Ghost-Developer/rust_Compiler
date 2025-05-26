use crate::lexer::Token;
use crate::ast::*;
use crate::error::CompilerError;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, expected: Token) -> Result<(), CompilerError> {
        if Some(&expected) == self.peek() {
            self.advance();
            Ok(())
        } else {
            Err(CompilerError::SyntaxError(format!(
                "Expected {:?}, found {:?}",
                expected,
                self.peek()
            )))
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, CompilerError> {
        let mut stmts = Vec::new();
        while self.peek().is_some() {
            stmts.push(self.parse_stmt()?);
        }
        Ok(stmts)
    }

    fn parse_stmt(&mut self) -> Result<Stmt, CompilerError> {
        match self.peek() {
            Some(Token::Let) => self.parse_let(),
            Some(Token::If) => self.parse_if(),
            Some(Token::While) => self.parse_while(),
            Some(Token::Do) => self.parse_do_while(),
            Some(Token::For) => self.parse_for(),
            Some(Token::Fn) => self.parse_fn_decl(),
            Some(Token::Return) => self.parse_return(),
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                if self.peek() == Some(&Token::Equal) {
                    self.advance();
                    let expr = self.parse_expr()?;
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::Assign(name, expr))
                } else {
                    // If it's not an assignment, treat it as an expression
                    let expr = Expr::Variable(name);
                    self.expect(Token::Semicolon)?;
                    Ok(Stmt::Expr(expr))
                }
            }
            _ => {
                let expr = self.parse_expr()?;
                self.expect(Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::Let)?;
        let name = if let Some(Token::Ident(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CompilerError::SyntaxError("Expected identifier after let".into()));
        };
        self.expect(Token::Equal)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Let(name, expr))
    }

    fn parse_if(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::If)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let then_block = self.parse_block()?;
        let else_block = if let Some(Token::Else) = self.peek() {
            self.advance();
            self.parse_block()?
        } else {
            Vec::new()
        };
        Ok(Stmt::If(cond, then_block, else_block))
    }

    fn parse_while(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::While(cond, body))
    }

    fn parse_do_while(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::Do)?;
        let body = self.parse_block()?;
        self.expect(Token::While)?;
        self.expect(Token::LParen)?;
        let cond = self.parse_expr()?;
        self.expect(Token::RParen)?;
        self.expect(Token::Semicolon)?;
        Ok(Stmt::DoWhile(body, cond))
    }

    fn parse_for(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::For)?;
        self.expect(Token::LParen)?;
        let var = if let Some(Token::Ident(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CompilerError::SyntaxError("Expected identifier in for loop".into()));
        };
        self.expect(Token::Equal)?;
        let start = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        let cond = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        let step = self.parse_expr()?;
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::For(var, start, cond, step, body))
    }

    fn parse_fn_decl(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::Fn)?;
        let name = if let Some(Token::Ident(name)) = self.peek() {
            let name = name.clone();
            self.advance();
            name
        } else {
            return Err(CompilerError::SyntaxError("Expected function name".into()));
        };
        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        if self.peek() != Some(&Token::RParen) {
            loop {
                if let Some(Token::Ident(param)) = self.peek() {
                    params.push(param.clone());
                    self.advance();
                } else {
                    return Err(CompilerError::SyntaxError("Expected parameter name".into()));
                }
                if self.peek() == Some(&Token::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.expect(Token::RParen)?;
        let body = self.parse_block()?;
        Ok(Stmt::FnDecl(name, params, body))
    }

    fn parse_return(&mut self) -> Result<Stmt, CompilerError> {
        self.expect(Token::Return)?;
        let expr = self.parse_expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt>, CompilerError> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        while self.peek() != Some(&Token::RBrace) {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(stmts)
    }

    fn parse_expr(&mut self) -> Result<Expr, CompilerError> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, CompilerError> {
        let mut expr = self.parse_comparison()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Eq | Token::Neq => {
                    let op = match token {
                        Token::Eq => BinOp::Eq,
                        Token::Neq => BinOp::Neq,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_comparison()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, CompilerError> {
        let mut expr = self.parse_term()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Gt | Token::Lt => {
                    let op = match token {
                        Token::Gt => BinOp::Gt,
                        Token::Lt => BinOp::Lt,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_term()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, CompilerError> {
        let mut expr = self.parse_factor()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = match token {
                        Token::Plus => BinOp::Add,
                        Token::Minus => BinOp::Sub,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_factor()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, CompilerError> {
        let mut expr = self.parse_unary()?;
        while let Some(token) = self.peek() {
            match token {
                Token::Star | Token::Slash => {
                    let op = match token {
                        Token::Star => BinOp::Mul,
                        Token::Slash => BinOp::Div,
                        _ => unreachable!(),
                    };
                    self.advance();
                    let right = self.parse_unary()?;
                    expr = Expr::Binary(Box::new(expr), op, Box::new(right));
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, CompilerError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                let expr = self.parse_primary()?;
                Ok(Expr::Binary(Box::new(Expr::Number(0)), BinOp::Sub, Box::new(expr)))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expr, CompilerError> {
        match self.peek() {
            Some(Token::Number(n)) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            Some(Token::True) => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            Some(Token::False) => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                if self.peek() == Some(&Token::LParen) {
                    // function call
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek() != Some(&Token::RParen) {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.peek() == Some(&Token::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call(name, args))
                } else {
                    Ok(Expr::Variable(name))
                }
            }
            Some(Token::LParen) => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            other => Err(CompilerError::SyntaxError(format!(
                "Unexpected token {:?} in expression",
                other
            ))),
        }
    }
}