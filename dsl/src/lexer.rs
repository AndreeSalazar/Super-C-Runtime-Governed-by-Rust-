//! SuperC DSL Lexer
//!
//! Tokeniza código fuente .sc en tokens para el parser.

use std::str::Chars;
use std::iter::Peekable;

/// Tipos de tokens del DSL
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Palabras clave
    Data,       // data
    Fn,         // fn
    Parallel,   // parallel
    Seq,        // seq
    Gpu,        // gpu
    Asm,        // asm
    Reduce,     // reduce
    If,         // if
    Else,       // else
    For,        // for
    Return,     // return
    
    // Tipos
    I32, I64, F32, F64, Bool,
    
    // Literales
    Ident(String),
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    
    // Operadores
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Eq,         // =
    EqEq,       // ==
    NotEq,      // !=
    Lt,         // <
    Gt,         // >
    LtEq,       // <=
    GtEq,       // >=
    And,        // &&
    Or,         // ||
    Not,        // !
    
    // Delimitadores
    LParen,     // (
    RParen,     // )
    LBrace,     // {
    RBrace,     // }
    LBracket,   // [
    RBracket,   // ]
    Comma,      // ,
    Colon,      // :
    Semi,       // ;
    Arrow,      // ->
    
    // Especiales
    Newline,
    Eof,
}

/// Token con posición
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

/// Lexer para SuperC DSL
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            input: source.chars().peekable(),
            line: 1,
            col: 1,
        }
    }
    
    fn advance(&mut self) -> Option<char> {
        let c = self.input.next();
        if let Some(ch) = c {
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }
    
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c == ' ' || c == '\t' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    fn skip_comment(&mut self) {
        // Skip // comments
        while let Some(&c) = self.peek() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }
    
    fn read_ident(&mut self, first: char) -> String {
        let mut s = String::new();
        s.push(first);
        while let Some(&c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        s
    }
    
    fn read_number(&mut self, first: char) -> TokenKind {
        let mut s = String::new();
        s.push(first);
        let mut is_float = false;
        
        while let Some(&c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        if is_float {
            TokenKind::FloatLit(s.parse().unwrap_or(0.0))
        } else {
            TokenKind::IntLit(s.parse().unwrap_or(0))
        }
    }
    
    fn read_string(&mut self) -> String {
        let mut s = String::new();
        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            if c == '\\' {
                if let Some(escaped) = self.advance() {
                    match escaped {
                        'n' => s.push('\n'),
                        't' => s.push('\t'),
                        '\\' => s.push('\\'),
                        '"' => s.push('"'),
                        _ => s.push(escaped),
                    }
                }
            } else {
                s.push(c);
            }
        }
        s
    }
    
    fn keyword_or_ident(&self, s: &str) -> TokenKind {
        match s {
            "data" => TokenKind::Data,
            "fn" => TokenKind::Fn,
            "parallel" => TokenKind::Parallel,
            "seq" => TokenKind::Seq,
            "gpu" => TokenKind::Gpu,
            "asm" => TokenKind::Asm,
            "reduce" => TokenKind::Reduce,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "return" => TokenKind::Return,
            "i32" => TokenKind::I32,
            "i64" => TokenKind::I64,
            "f32" => TokenKind::F32,
            "f64" => TokenKind::F64,
            "bool" => TokenKind::Bool,
            "true" => TokenKind::BoolLit(true),
            "false" => TokenKind::BoolLit(false),
            _ => TokenKind::Ident(s.to_string()),
        }
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let line = self.line;
        let col = self.col;
        
        let kind = match self.advance() {
            None => TokenKind::Eof,
            Some(c) => match c {
                '\n' => TokenKind::Newline,
                
                // Comments
                '/' if self.peek() == Some(&'/') => {
                    self.skip_comment();
                    return self.next_token();
                }
                
                // Operators
                '+' => TokenKind::Plus,
                '-' if self.peek() == Some(&'>') => { self.advance(); TokenKind::Arrow }
                '-' => TokenKind::Minus,
                '*' => TokenKind::Star,
                '/' => TokenKind::Slash,
                '%' => TokenKind::Percent,
                '=' if self.peek() == Some(&'=') => { self.advance(); TokenKind::EqEq }
                '=' => TokenKind::Eq,
                '!' if self.peek() == Some(&'=') => { self.advance(); TokenKind::NotEq }
                '!' => TokenKind::Not,
                '<' if self.peek() == Some(&'=') => { self.advance(); TokenKind::LtEq }
                '<' => TokenKind::Lt,
                '>' if self.peek() == Some(&'=') => { self.advance(); TokenKind::GtEq }
                '>' => TokenKind::Gt,
                '&' if self.peek() == Some(&'&') => { self.advance(); TokenKind::And }
                '|' if self.peek() == Some(&'|') => { self.advance(); TokenKind::Or }
                
                // Delimiters
                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,
                ',' => TokenKind::Comma,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semi,
                
                // String
                '"' => TokenKind::StringLit(self.read_string()),
                
                // Number
                c if c.is_ascii_digit() => self.read_number(c),
                
                // Identifier
                c if c.is_alphabetic() || c == '_' => {
                    let s = self.read_ident(c);
                    self.keyword_or_ident(&s)
                }
                
                _ => return self.next_token(), // Skip unknown
            }
        };
        
        Token { kind, line, col }
    }
    
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = tok.kind == TokenKind::Eof;
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let mut lexer = Lexer::new("data x: f32[100]");
        let tokens = lexer.tokenize();
        assert!(matches!(tokens[0].kind, TokenKind::Data));
        assert!(matches!(tokens[1].kind, TokenKind::Ident(_)));
    }
}
