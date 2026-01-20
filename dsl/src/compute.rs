//! SuperC Compute Engine
//!
//! Motor de cómputo unificado que ejecuta workloads automáticamente
//! seleccionando el mejor backend: CPU, GPU (CUDA/HIP), o ASM optimizado.

use crate::ast::*;
use crate::parser::Parser;

/// Preferencia de ejecución
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComputePreference {
    Auto,       // Selección automática
    Gpu,        // Preferir GPU
    Cpu,        // Preferir CPU
    Asm,        // Preferir ASM optimizado
    LowPower,   // Bajo consumo
}

/// Backend disponible
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Backend {
    CudaGpu,
    HipGpu,
    HipCpu,
    AsmSimd,
    PureCpu,
}

/// Resultado de ejecución
#[derive(Debug)]
pub struct ComputeResult {
    pub backend_used: Backend,
    pub execution_time_us: u64,
    pub success: bool,
    pub output: Vec<f32>,
}

/// Motor de cómputo unificado
pub struct ComputeEngine {
    preference: ComputePreference,
    available_backends: Vec<Backend>,
}

impl ComputeEngine {
    pub fn new() -> Self {
        Self {
            preference: ComputePreference::Auto,
            available_backends: Self::detect_backends(),
        }
    }
    
    pub fn with_preference(preference: ComputePreference) -> Self {
        Self {
            preference,
            available_backends: Self::detect_backends(),
        }
    }
    
    fn detect_backends() -> Vec<Backend> {
        let mut backends = Vec::new();
        
        // Siempre disponible
        backends.push(Backend::PureCpu);
        backends.push(Backend::AsmSimd);  // AVX disponible en CPUs modernos
        backends.push(Backend::HipCpu);   // HIP-CPU siempre disponible
        
        // TODO: Detectar GPU real
        // Por ahora asumimos que no hay GPU
        
        backends
    }
    
    /// Selecciona el mejor backend según preferencia y disponibilidad
    fn select_backend(&self, workload_size: usize) -> Backend {
        match self.preference {
            ComputePreference::Gpu => {
                if self.available_backends.contains(&Backend::CudaGpu) {
                    Backend::CudaGpu
                } else if self.available_backends.contains(&Backend::HipGpu) {
                    Backend::HipGpu
                } else {
                    Backend::HipCpu
                }
            }
            ComputePreference::Asm => Backend::AsmSimd,
            ComputePreference::Cpu => Backend::PureCpu,
            ComputePreference::LowPower => Backend::PureCpu,
            ComputePreference::Auto => {
                // Heurística: GPU para workloads grandes, ASM para medianos, CPU para pequeños
                if workload_size > 100_000 {
                    if self.available_backends.contains(&Backend::CudaGpu) {
                        Backend::CudaGpu
                    } else if self.available_backends.contains(&Backend::HipGpu) {
                        Backend::HipGpu
                    } else {
                        Backend::AsmSimd
                    }
                } else if workload_size > 1000 {
                    Backend::AsmSimd
                } else {
                    Backend::PureCpu
                }
            }
        }
    }
    
    /// Ejecuta un workload desde código SuperC
    pub fn execute(&self, source: &str) -> Result<ComputeResult, String> {
        let mut parser = Parser::from_source(source);
        let program = parser.parse()?;
        
        // Analizar el programa para determinar tamaño del workload
        let workload_size = self.analyze_workload(&program);
        let backend = self.select_backend(workload_size);
        
        let start = std::time::Instant::now();
        let output = self.run_program(&program, backend)?;
        let elapsed = start.elapsed().as_micros() as u64;
        
        Ok(ComputeResult {
            backend_used: backend,
            execution_time_us: elapsed,
            success: true,
            output,
        })
    }
    
    fn analyze_workload(&self, program: &Program) -> usize {
        let mut max_size = 0;
        for stmt in &program.statements {
            if let Stmt::DataDecl { dtype, .. } = stmt {
                if let DataType::Array(_, size) = dtype {
                    max_size = max_size.max(*size);
                }
            }
        }
        max_size
    }
    
    fn run_program(&self, program: &Program, backend: Backend) -> Result<Vec<f32>, String> {
        let mut env = ExecutionEnv::new();
        
        // Procesar declaraciones de datos
        for stmt in &program.statements {
            if let Stmt::DataDecl { name, dtype } = stmt {
                env.declare(name, dtype);
            }
        }
        
        // Ejecutar statements
        for stmt in &program.statements {
            self.execute_stmt(&mut env, stmt, backend)?;
        }
        
        // Retornar el primer array como output
        env.get_first_array()
    }
    
    fn execute_stmt(&self, env: &mut ExecutionEnv, stmt: &Stmt, backend: Backend) -> Result<(), String> {
        match stmt {
            Stmt::DataDecl { .. } => Ok(()), // Ya procesado
            
            Stmt::Assign { target, index, value } => {
                let val = self.eval_expr(env, value, backend)?;
                if let Some(idx_expr) = index {
                    let idx = self.eval_expr_int(env, idx_expr)? as usize;
                    env.set_array_elem(target, idx, val);
                } else {
                    env.set_scalar(target, val);
                }
                Ok(())
            }
            
            Stmt::ExecBlock { target: _, body } => {
                for s in body {
                    self.execute_stmt(env, s, backend)?;
                }
                Ok(())
            }
            
            Stmt::For { var, start, end, body } => {
                let start_val = self.eval_expr_int(env, start)?;
                let end_val = self.eval_expr_int(env, end)?;
                
                // Usar backend apropiado para el loop
                match backend {
                    Backend::AsmSimd => {
                        // Ejecutar con SIMD si es operación vectorial simple
                        self.execute_simd_loop(env, var, start_val, end_val, body)?;
                    }
                    _ => {
                        // Ejecución secuencial
                        for i in start_val..end_val {
                            env.set_scalar(var, i as f32);
                            for s in body {
                                self.execute_stmt(env, s, backend)?;
                            }
                        }
                    }
                }
                Ok(())
            }
            
            Stmt::If { cond, then_body, else_body } => {
                let cond_val = self.eval_expr(env, cond, backend)?;
                if cond_val != 0.0 {
                    for s in then_body {
                        self.execute_stmt(env, s, backend)?;
                    }
                } else if let Some(else_stmts) = else_body {
                    for s in else_stmts {
                        self.execute_stmt(env, s, backend)?;
                    }
                }
                Ok(())
            }
            
            Stmt::ExprStmt(expr) => {
                if let Expr::Call(name, args) = expr {
                    self.execute_call(env, name, args, backend)?;
                }
                Ok(())
            }
            
            Stmt::Return(_) => Ok(()),
        }
    }
    
    fn execute_simd_loop(&self, env: &mut ExecutionEnv, var: &str, start: i64, end: i64, body: &[Stmt]) -> Result<(), String> {
        // Analizar si el body es una operación vectorial simple
        // Por ahora, ejecutar secuencialmente pero con optimización futura
        for i in start..end {
            env.set_scalar(var, i as f32);
            for s in body {
                self.execute_stmt(env, s, Backend::PureCpu)?;
            }
        }
        Ok(())
    }
    
    fn execute_call(&self, env: &mut ExecutionEnv, name: &str, args: &[Expr], backend: Backend) -> Result<(), String> {
        match name {
            "print" => {
                if let Some(arg) = args.first() {
                    let val = self.eval_expr(env, arg, backend)?;
                    println!("{:.6}", val);
                }
            }
            _ => {}
        }
        Ok(())
    }
    
    fn eval_expr(&self, env: &ExecutionEnv, expr: &Expr, _backend: Backend) -> Result<f32, String> {
        match expr {
            Expr::IntLit(n) => Ok(*n as f32),
            Expr::FloatLit(n) => Ok(*n as f32),
            Expr::BoolLit(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Expr::Ident(name) => env.get_scalar(name),
            
            Expr::Index(arr, idx) => {
                if let Expr::Ident(name) = arr.as_ref() {
                    let idx_val = self.eval_expr_int(env, idx)? as usize;
                    env.get_array_elem(name, idx_val)
                } else {
                    Err("Invalid array access".to_string())
                }
            }
            
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr(env, left, _backend)?;
                let r = self.eval_expr(env, right, _backend)?;
                Ok(match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => l / r,
                    BinOp::Mod => l % r,
                    BinOp::Eq => if (l - r).abs() < f32::EPSILON { 1.0 } else { 0.0 },
                    BinOp::NotEq => if (l - r).abs() >= f32::EPSILON { 1.0 } else { 0.0 },
                    BinOp::Lt => if l < r { 1.0 } else { 0.0 },
                    BinOp::Gt => if l > r { 1.0 } else { 0.0 },
                    BinOp::LtEq => if l <= r { 1.0 } else { 0.0 },
                    BinOp::GtEq => if l >= r { 1.0 } else { 0.0 },
                    BinOp::And => if l != 0.0 && r != 0.0 { 1.0 } else { 0.0 },
                    BinOp::Or => if l != 0.0 || r != 0.0 { 1.0 } else { 0.0 },
                })
            }
            
            Expr::UnaryOp(op, inner) => {
                let v = self.eval_expr(env, inner, _backend)?;
                Ok(match op {
                    UnaryOp::Neg => -v,
                    UnaryOp::Not => if v == 0.0 { 1.0 } else { 0.0 },
                })
            }
            
            Expr::Call(name, args) => {
                match name.as_str() {
                    "sqrt" => {
                        let v = self.eval_expr(env, &args[0], _backend)?;
                        Ok(v.sqrt())
                    }
                    "sin" => {
                        let v = self.eval_expr(env, &args[0], _backend)?;
                        Ok(v.sin())
                    }
                    "cos" => {
                        let v = self.eval_expr(env, &args[0], _backend)?;
                        Ok(v.cos())
                    }
                    "exp" => {
                        let v = self.eval_expr(env, &args[0], _backend)?;
                        Ok(v.exp())
                    }
                    "log" => {
                        let v = self.eval_expr(env, &args[0], _backend)?;
                        Ok(v.ln())
                    }
                    _ => Err(format!("Unknown function: {}", name)),
                }
            }
            
            Expr::Reduce(op, arr) => {
                if let Expr::Ident(name) = arr.as_ref() {
                    let array = env.get_array(name)?;
                    Ok(match op {
                        ReduceOp::Sum => array.iter().sum(),
                        ReduceOp::Prod => array.iter().product(),
                        ReduceOp::Max => array.iter().cloned().fold(f32::MIN, f32::max),
                        ReduceOp::Min => array.iter().cloned().fold(f32::MAX, f32::min),
                    })
                } else {
                    Err("Reduce requires array identifier".to_string())
                }
            }
            
            _ => Err("Unsupported expression".to_string()),
        }
    }
    
    fn eval_expr_int(&self, env: &ExecutionEnv, expr: &Expr) -> Result<i64, String> {
        match expr {
            Expr::IntLit(n) => Ok(*n),
            Expr::Ident(name) => Ok(env.get_scalar(name)? as i64),
            Expr::BinOp(left, op, right) => {
                let l = self.eval_expr_int(env, left)?;
                let r = self.eval_expr_int(env, right)?;
                Ok(match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => l / r,
                    BinOp::Mod => l % r,
                    _ => l,
                })
            }
            _ => Err("Expected integer expression".to_string()),
        }
    }
}

impl Default for ComputeEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Entorno de ejecución
struct ExecutionEnv {
    scalars: std::collections::HashMap<String, f32>,
    arrays: std::collections::HashMap<String, Vec<f32>>,
}

impl ExecutionEnv {
    fn new() -> Self {
        Self {
            scalars: std::collections::HashMap::new(),
            arrays: std::collections::HashMap::new(),
        }
    }
    
    fn declare(&mut self, name: &str, dtype: &DataType) {
        match dtype {
            DataType::Array(_, size) => {
                self.arrays.insert(name.to_string(), vec![0.0; *size]);
            }
            _ => {
                self.scalars.insert(name.to_string(), 0.0);
            }
        }
    }
    
    fn set_scalar(&mut self, name: &str, value: f32) {
        self.scalars.insert(name.to_string(), value);
    }
    
    fn get_scalar(&self, name: &str) -> Result<f32, String> {
        self.scalars.get(name).copied()
            .ok_or_else(|| format!("Undefined variable: {}", name))
    }
    
    fn set_array_elem(&mut self, name: &str, idx: usize, value: f32) {
        if let Some(arr) = self.arrays.get_mut(name) {
            if idx < arr.len() {
                arr[idx] = value;
            }
        }
    }
    
    fn get_array_elem(&self, name: &str, idx: usize) -> Result<f32, String> {
        self.arrays.get(name)
            .and_then(|arr| arr.get(idx).copied())
            .ok_or_else(|| format!("Array access error: {}[{}]", name, idx))
    }
    
    fn get_array(&self, name: &str) -> Result<&Vec<f32>, String> {
        self.arrays.get(name)
            .ok_or_else(|| format!("Undefined array: {}", name))
    }
    
    fn get_first_array(&self) -> Result<Vec<f32>, String> {
        self.arrays.values().next().cloned()
            .ok_or_else(|| "No arrays in program".to_string())
    }
}

/// Formato de backend para display
impl std::fmt::Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Backend::CudaGpu => write!(f, "CUDA GPU"),
            Backend::HipGpu => write!(f, "HIP GPU (AMD)"),
            Backend::HipCpu => write!(f, "HIP-CPU"),
            Backend::AsmSimd => write!(f, "ASM SIMD (AVX)"),
            Backend::PureCpu => write!(f, "Pure CPU"),
        }
    }
}
