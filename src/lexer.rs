use crate::error::CompilerError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Fn,
    If,
    Else,
    While,
    Do,
    For,
    Return,
    True,
    False,
    Ident(String),
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Eq,
    Neq,
    Gt,
    Lt,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    Comma,
    Colon,   // <--- Added Colon token here
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, CompilerError> {
        let mut tokens = Vec::new();
        while let Some(&c) = self.peek() {
            match c {
                ' ' | '\n' | '\t' | '\r' => {
                    self.advance();
                }
                '0'..='9' => tokens.push(self.tokenize_number()?),
                'a'..='z' | 'A'..='Z' | '_' => tokens.push(self.tokenize_ident_or_keyword()?),
                '+' => {
                    self.advance();
                    tokens.push(Token::Plus);
                }
                '-' => {
                    self.advance();
                    tokens.push(Token::Minus);
                }
                '*' => {
                    self.advance();
                    tokens.push(Token::Star);
                }
                '/' => {
                    self.advance();
                    tokens.push(Token::Slash);
                }
                '=' => {
                    self.advance();
                    if self.match_char('=') {
                        tokens.push(Token::Eq);
                    } else {
                        tokens.push(Token::Equal);
                    }
                }
                '!' => {
                    self.advance();
                    if self.match_char('=') {
                        tokens.push(Token::Neq);
                    } else {
                        return Err(CompilerError::SyntaxError("Unexpected character after '!'".into()));
                    }
                }
                '>' => {
                    self.advance();
                    tokens.push(Token::Gt);
                }
                '<' => {
                    self.advance();
                    tokens.push(Token::Lt);
                }
                '(' => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                ')' => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                '{' => {
                    self.advance();
                    tokens.push(Token::LBrace);
                }
                '}' => {
                    self.advance();
                    tokens.push(Token::RBrace);
                }
                ';' => {
                    self.advance();
                    tokens.push(Token::Semicolon);
                }
                ',' => {
                    self.advance();
                    tokens.push(Token::Comma);
                }
                ':' => {                   // <--- Added this block
                    self.advance();
                    tokens.push(Token::Colon);
                }
                _ => {
                    return Err(CompilerError::SyntaxError(format!("Unexpected character: {}", c)));
                }
            }
        }
        Ok(tokens)
    }

    fn tokenize_number(&mut self) -> Result<Token, CompilerError> {
        let mut num = 0i64;
        while let Some(&c) = self.peek() {
            if let Some(d) = c.to_digit(10) {
                num = num * 10 + d as i64;
                self.advance();
            } else {
                break;
            }
        }
        Ok(Token::Number(num))
    }

    fn tokenize_ident_or_keyword(&mut self) -> Result<Token, CompilerError> {
        let mut ident = String::new();
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Ok(match ident.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "do" => Token::Do,
            "for" => Token::For,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        })
    }

    fn peek(&self) -> Option<&char> {
        self.input.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn match_char(&mut self, expected: char) -> bool {
        if let Some(&c) = self.peek() {
            if c == expected {
                self.advance();
                return true;
            }
        }
        false
    }
}
