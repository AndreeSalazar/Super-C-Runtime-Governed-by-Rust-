//! SuperC DSL Abstract Syntax Tree
//!
//! Representación del código parseado.

/// Tipos de datos del DSL
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    I32,
    I64,
    F32,
    F64,
    Bool,
    Array(Box<DataType>, usize), // tipo[tamaño]
    Void,
}

/// Operadores binarios
#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    Eq,     // ==
    NotEq,  // !=
    Lt,     // <
    Gt,     // >
    LtEq,   // <=
    GtEq,   // >=
    And,    // &&
    Or,     // ||
}

/// Operadores unarios
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // !
}

/// Funciones de reducción
#[derive(Debug, Clone, PartialEq)]
pub enum ReduceOp {
    Sum,
    Max,
    Min,
    Prod,
}

/// Expresiones
#[derive(Debug, Clone)]
pub enum Expr {
    // Literales
    IntLit(i64),
    FloatLit(f64),
    BoolLit(bool),
    StringLit(String),
    
    // Variables
    Ident(String),
    
    // Acceso a array
    Index(Box<Expr>, Box<Expr>), // array[index]
    
    // Operaciones
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    
    // Llamada a función
    Call(String, Vec<Expr>),
    
    // Reducción
    Reduce(ReduceOp, Box<Expr>),
}

/// Objetivo de ejecución
#[derive(Debug, Clone, PartialEq)]
pub enum ExecTarget {
    Parallel,   // Auto GPU/CPU
    Seq,        // CPU secuencial
    Gpu,        // Forzar GPU
    Asm,        // CPU con ASM
}

/// Declaraciones/Statements
#[derive(Debug, Clone)]
pub enum Stmt {
    // Declaración de datos
    DataDecl {
        name: String,
        dtype: DataType,
    },
    
    // Asignación
    Assign {
        target: String,
        index: Option<Expr>,  // Para array[i] = ...
        value: Expr,
    },
    
    // Bloque de ejecución
    ExecBlock {
        target: ExecTarget,
        body: Vec<Stmt>,
    },
    
    // Condicional
    If {
        cond: Expr,
        then_body: Vec<Stmt>,
        else_body: Option<Vec<Stmt>>,
    },
    
    // Bucle for
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },
    
    // Llamada a función (como statement)
    ExprStmt(Expr),
    
    // Return
    Return(Option<Expr>),
}

/// Parámetro de función
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub dtype: DataType,
}

/// Definición de función
#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<Param>,
    pub ret_type: DataType,
    pub body: Vec<Stmt>,
}

/// Programa completo
#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<FnDef>,
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            statements: Vec::new(),
        }
    }
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}
