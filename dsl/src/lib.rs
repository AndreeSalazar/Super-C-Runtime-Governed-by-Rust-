//! SuperC DSL - Domain Specific Language
//!
//! Un lenguaje simplificado que se traduce a Rust → C → CUDA/HIP/ASM
//!
//! ```superc
//! data vec: f32[1000]
//! parallel {
//!     vec = vec + 1.0
//! }
//! ```

pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
pub mod codegen_asm;
pub mod compute;

pub use lexer::Lexer;
pub use parser::Parser;
pub use codegen::{Codegen, CodegenTarget};
pub use codegen_asm::AsmCodegen;
pub use compute::{ComputeEngine, ComputePreference, Backend, ComputeResult};
pub use ast::Program;

/// Compila código SuperC a Rust
pub fn compile_to_rust(source: &str) -> Result<String, String> {
    let mut parser = Parser::from_source(source);
    let program = parser.parse()?;
    let mut codegen = Codegen::new(CodegenTarget::Rust);
    Ok(codegen.generate(&program))
}

/// Compila código SuperC a C
pub fn compile_to_c(source: &str) -> Result<String, String> {
    let mut parser = Parser::from_source(source);
    let program = parser.parse()?;
    let mut codegen = Codegen::new(CodegenTarget::C);
    Ok(codegen.generate(&program))
}

/// Compila código SuperC a ASM (NASM Windows x64)
pub fn compile_to_asm(source: &str) -> Result<String, String> {
    let mut parser = Parser::from_source(source);
    let program = parser.parse()?;
    let mut codegen = AsmCodegen::new();
    Ok(codegen.generate(&program))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_compile() {
        let source = r#"
            data x: f32[100]
            parallel {
                x[0] = 42.0
            }
        "#;
        
        let rust_code = compile_to_rust(source).unwrap();
        assert!(rust_code.contains("let mut x"));
        
        let c_code = compile_to_c(source).unwrap();
        assert!(c_code.contains("float x[100]"));
    }
}
