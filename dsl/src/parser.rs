//! SuperC DSL Parser
//!
//! Convierte tokens en AST.

use crate::lexer::{Token, TokenKind, Lexer};
use crate::ast::*;

/// Parser para SuperC DSL
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    
    pub fn from_source(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        Self::new(tokens)
    }
    
    fn current(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            line: 0,
            col: 0,
        })
    }
    
    fn peek(&self) -> TokenKind {
        self.current().kind
    }
    
    fn advance(&mut self) -> Token {
        let tok = self.current();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        // Skip newlines
        while matches!(self.peek(), TokenKind::Newline) {
            self.pos += 1;
        }
        tok
    }
    
    fn expect(&mut self, kind: TokenKind) -> Result<(), String> {
        if std::mem::discriminant(&self.peek()) == std::mem::discriminant(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", kind, self.peek()))
        }
    }
    
    fn skip_newlines(&mut self) {
        while matches!(self.peek(), TokenKind::Newline) {
            self.pos += 1;
        }
    }
    
    /// Parse programa completo
    pub fn parse(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        
        self.skip_newlines();
        
        while !matches!(self.peek(), TokenKind::Eof) {
            self.skip_newlines();
            
            match self.peek() {
                TokenKind::Fn => {
                    let func = self.parse_function()?;
                    program.functions.push(func);
                }
                TokenKind::Eof => break,
                _ => {
                    let stmt = self.parse_statement()?;
                    program.statements.push(stmt);
                }
            }
            
            self.skip_newlines();
        }
        
        Ok(program)
    }
    
    /// Parse función
    fn parse_function(&mut self) -> Result<FnDef, String> {
        self.expect(TokenKind::Fn)?;
        
        let name = match self.peek().clone() {
            TokenKind::Ident(s) => { self.advance(); s }
            _ => return Err("Expected function name".to_string()),
        };
        
        self.expect(TokenKind::LParen)?;
        
        let mut params = Vec::new();
        while !matches!(self.peek(), TokenKind::RParen) {
            let param_name = match self.peek().clone() {
                TokenKind::Ident(s) => { self.advance(); s }
                _ => return Err("Expected parameter name".to_string()),
            };
            self.expect(TokenKind::Colon)?;
            let dtype = self.parse_type()?;
            params.push(Param { name: param_name, dtype });
            
            if matches!(self.peek(), TokenKind::Comma) {
                self.advance();
            }
        }
        self.expect(TokenKind::RParen)?;
        
        let ret_type = if matches!(self.peek(), TokenKind::Arrow) {
            self.advance();
            self.parse_type()?
        } else {
            DataType::Void
        };
        
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;
        
        Ok(FnDef { name, params, ret_type, body })
    }
    
    /// Parse tipo de dato
    fn parse_type(&mut self) -> Result<DataType, String> {
        let base = match self.peek() {
            TokenKind::I32 => { self.advance(); DataType::I32 }
            TokenKind::I64 => { self.advance(); DataType::I64 }
            TokenKind::F32 => { self.advance(); DataType::F32 }
            TokenKind::F64 => { self.advance(); DataType::F64 }
            TokenKind::Bool => { self.advance(); DataType::Bool }
            _ => return Err(format!("Expected type, got {:?}", self.peek())),
        };
        
        // Check for array
        if matches!(self.peek(), TokenKind::LBracket) {
            self.advance();
            let size = match self.peek() {
                TokenKind::IntLit(n) => { let n = n as usize; self.advance(); n }
                _ => return Err("Expected array size".to_string()),
            };
            self.expect(TokenKind::RBracket)?;
            Ok(DataType::Array(Box::new(base), size))
        } else {
            Ok(base)
        }
    }
    
    /// Parse bloque de statements
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        
        while !matches!(self.peek(), TokenKind::RBrace | TokenKind::Eof) {
            stmts.push(self.parse_statement()?);
            self.skip_newlines();
        }
        
        Ok(stmts)
    }
    
    /// Parse statement
    fn parse_statement(&mut self) -> Result<Stmt, String> {
        self.skip_newlines();
        
        match self.peek().clone() {
            TokenKind::Data => self.parse_data_decl(),
            TokenKind::Parallel => self.parse_exec_block(ExecTarget::Parallel),
            TokenKind::Seq => self.parse_exec_block(ExecTarget::Seq),
            TokenKind::Gpu => self.parse_exec_block(ExecTarget::Gpu),
            TokenKind::Asm => self.parse_exec_block(ExecTarget::Asm),
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Ident(_) => self.parse_assign_or_expr(),
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }
    
    /// Parse declaración de datos
    fn parse_data_decl(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Data)?;
        
        let name = match self.peek().clone() {
            TokenKind::Ident(s) => { self.advance(); s }
            _ => return Err("Expected variable name".to_string()),
        };
        
        self.expect(TokenKind::Colon)?;
        let dtype = self.parse_type()?;
        
        Ok(Stmt::DataDecl { name, dtype })
    }
    
    /// Parse bloque de ejecución
    fn parse_exec_block(&mut self, target: ExecTarget) -> Result<Stmt, String> {
        self.advance(); // consume keyword
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;
        
        Ok(Stmt::ExecBlock { target, body })
    }
    
    /// Parse if
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::If)?;
        let cond = self.parse_expr()?;
        self.expect(TokenKind::LBrace)?;
        let then_body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;
        
        let else_body = if matches!(self.peek(), TokenKind::Else) {
            self.advance();
            self.expect(TokenKind::LBrace)?;
            let body = self.parse_block()?;
            self.expect(TokenKind::RBrace)?;
            Some(body)
        } else {
            None
        };
        
        Ok(Stmt::If { cond, then_body, else_body })
    }
    
    /// Parse for
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::For)?;
        
        let var = match self.peek().clone() {
            TokenKind::Ident(s) => { self.advance(); s }
            _ => return Err("Expected loop variable".to_string()),
        };
        
        self.expect(TokenKind::Eq)?;
        let start = self.parse_expr()?;
        
        // Expect ".." or "to"
        self.expect(TokenKind::Colon)?;
        let end = self.parse_expr()?;
        
        self.expect(TokenKind::LBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenKind::RBrace)?;
        
        Ok(Stmt::For { var, start, end, body })
    }
    
    /// Parse return
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::Return)?;
        
        let value = if matches!(self.peek(), TokenKind::Newline | TokenKind::RBrace | TokenKind::Eof) {
            None
        } else {
            Some(self.parse_expr()?)
        };
        
        Ok(Stmt::Return(value))
    }
    
    /// Parse asignación o expresión
    fn parse_assign_or_expr(&mut self) -> Result<Stmt, String> {
        let name = match self.peek().clone() {
            TokenKind::Ident(s) => { self.advance(); s }
            _ => return Err("Expected identifier".to_string()),
        };
        
        // Check for array index
        let index = if matches!(self.peek(), TokenKind::LBracket) {
            self.advance();
            let idx = self.parse_expr()?;
            self.expect(TokenKind::RBracket)?;
            Some(idx)
        } else {
            None
        };
        
        if matches!(self.peek(), TokenKind::Eq) {
            self.advance();
            let value = self.parse_expr()?;
            Ok(Stmt::Assign { target: name, index, value })
        } else {
            // Es una llamada a función o expresión
            let expr = if matches!(self.peek(), TokenKind::LParen) {
                self.advance();
                let mut args = Vec::new();
                while !matches!(self.peek(), TokenKind::RParen) {
                    args.push(self.parse_expr()?);
                    if matches!(self.peek(), TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RParen)?;
                Expr::Call(name, args)
            } else {
                Expr::Ident(name)
            };
            Ok(Stmt::ExprStmt(expr))
        }
    }
    
    /// Parse expresión
    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }
    
    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while matches!(self.peek(), TokenKind::Or) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp(Box::new(left), BinOp::Or, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_equality()?;
        while matches!(self.peek(), TokenKind::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinOp(Box::new(left), BinOp::And, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                TokenKind::EqEq => BinOp::Eq,
                TokenKind::NotEq => BinOp::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_additive()?;
        loop {
            let op = match self.peek() {
                TokenKind::Lt => BinOp::Lt,
                TokenKind::Gt => BinOp::Gt,
                TokenKind::LtEq => BinOp::LtEq,
                TokenKind::GtEq => BinOp::GtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_additive(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match self.peek() {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_multiplicative(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            let op = match self.peek() {
                TokenKind::Star => BinOp::Mul,
                TokenKind::Slash => BinOp::Div,
                TokenKind::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }
    
    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp(UnaryOp::Neg, Box::new(expr)))
            }
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp(UnaryOp::Not, Box::new(expr)))
            }
            _ => self.parse_primary(),
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            TokenKind::IntLit(n) => { self.advance(); Ok(Expr::IntLit(n)) }
            TokenKind::FloatLit(n) => { self.advance(); Ok(Expr::FloatLit(n)) }
            TokenKind::BoolLit(b) => { self.advance(); Ok(Expr::BoolLit(b)) }
            TokenKind::StringLit(s) => { self.advance(); Ok(Expr::StringLit(s)) }
            
            TokenKind::Reduce => {
                self.advance();
                self.expect(TokenKind::LParen)?;
                let op = match self.peek() {
                    TokenKind::Plus => { self.advance(); ReduceOp::Sum }
                    TokenKind::Star => { self.advance(); ReduceOp::Prod }
                    TokenKind::Ident(ref s) if s == "max" => { self.advance(); ReduceOp::Max }
                    TokenKind::Ident(ref s) if s == "min" => { self.advance(); ReduceOp::Min }
                    _ => return Err("Expected reduce operation".to_string()),
                };
                self.expect(TokenKind::Comma)?;
                let arr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expr::Reduce(op, Box::new(arr)))
            }
            
            TokenKind::Ident(name) => {
                self.advance();
                
                // Function call or array index
                if matches!(self.peek(), TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    while !matches!(self.peek(), TokenKind::RParen) {
                        args.push(self.parse_expr()?);
                        if matches!(self.peek(), TokenKind::Comma) {
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Call(name, args))
                } else if matches!(self.peek(), TokenKind::LBracket) {
                    self.advance();
                    let idx = self.parse_expr()?;
                    self.expect(TokenKind::RBracket)?;
                    Ok(Expr::Index(Box::new(Expr::Ident(name)), Box::new(idx)))
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            
            _ => Err(format!("Unexpected token in expression: {:?}", self.peek())),
        }
    }
}
