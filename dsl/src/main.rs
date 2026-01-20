//! SuperC DSL Compiler CLI
//!
//! Uso:
//!   superc build ejemplo.sc
//!   superc run ejemplo.sc
//!   superc emit ejemplo.sc --target=rust

use std::env;
use std::fs;
use std::path::Path;

use superc_dsl::{compile_to_rust, compile_to_c, compile_to_asm, ComputeEngine, ComputePreference};

fn print_usage() {
    println!("SuperC Compute Engine - Motor de Cómputo Unificado");
    println!("==================================================");
    println!("");
    println!("Uso:");
    println!("  superc run <archivo.sc> [--gpu|--cpu|--asm]   Ejecuta directamente");
    println!("  superc emit <archivo.sc> --rust               Emite código Rust");
    println!("  superc emit <archivo.sc> --c                  Emite código C");
    println!("  superc emit <archivo.sc> --asm                Emite código ASM (NASM Win64)");
    println!("  superc build <archivo.sc>                     Compila a ejecutable");
    println!("");
    println!("Opciones de ejecución:");
    println!("  --gpu    Preferir GPU (CUDA/HIP/HIP-CPU)");
    println!("  --cpu    Preferir CPU puro");
    println!("  --asm    Preferir ASM SIMD optimizado");
    println!("  (sin opción = selección automática)");
    println!("");
    println!("Ejemplos:");
    println!("  superc run ejemplo.sc              # Auto-selección");
    println!("  superc run ejemplo.sc --gpu        # Forzar GPU/HIP-CPU");
    println!("  superc run ejemplo.sc --asm        # Forzar ASM SIMD");
    println!("  superc emit ejemplo.sc --rust      # Ver código Rust");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "build" => {
            if args.len() < 3 {
                eprintln!("Error: Falta archivo de entrada");
                return;
            }
            let file = &args[2];
            build_file(file);
        }
        
        "run" => {
            if args.len() < 3 {
                eprintln!("Error: Falta archivo de entrada");
                return;
            }
            let file = &args[2];
            let pref = if args.len() > 3 {
                match args[3].as_str() {
                    "--gpu" => ComputePreference::Gpu,
                    "--cpu" => ComputePreference::Cpu,
                    "--asm" => ComputePreference::Asm,
                    _ => ComputePreference::Auto,
                }
            } else {
                ComputePreference::Auto
            };
            run_file_with_pref(file, pref);
        }
        
        "emit" => {
            if args.len() < 4 {
                eprintln!("Error: Falta archivo o target");
                eprintln!("Uso: superc emit <archivo.sc> --rust|--c");
                return;
            }
            let file = &args[2];
            let target = &args[3];
            emit_file(file, target);
        }
        
        "help" | "--help" | "-h" => {
            print_usage();
        }
        
        _ => {
            eprintln!("Comando desconocido: {}", command);
            print_usage();
        }
    }
}

fn read_source(file: &str) -> Result<String, String> {
    let path = Path::new(file);
    if !path.exists() {
        return Err(format!("Archivo no encontrado: {}", file));
    }
    fs::read_to_string(path).map_err(|e| format!("Error leyendo archivo: {}", e))
}

fn build_file(file: &str) {
    println!("🔨 Compilando {}...", file);
    
    let source = match read_source(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {}", e);
            return;
        }
    };
    
    // Generar código Rust
    match compile_to_rust(&source) {
        Ok(rust_code) => {
            let out_file = file.replace(".sc", "_generated.rs");
            if let Err(e) = fs::write(&out_file, &rust_code) {
                eprintln!("❌ Error escribiendo {}: {}", out_file, e);
                return;
            }
            println!("✅ Generado: {}", out_file);
            
            // También generar C
            if let Ok(c_code) = compile_to_c(&source) {
                let c_out = file.replace(".sc", "_generated.c");
                if let Err(e) = fs::write(&c_out, &c_code) {
                    eprintln!("⚠️  Error escribiendo {}: {}", c_out, e);
                } else {
                    println!("✅ Generado: {}", c_out);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error de compilación: {}", e);
        }
    }
}

fn run_file_with_pref(file: &str, preference: ComputePreference) {
    println!("🚀 SuperC Compute Engine");
    println!("========================");
    println!("📂 Archivo: {}", file);
    
    let source = match read_source(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {}", e);
            return;
        }
    };
    
    let engine = ComputeEngine::with_preference(preference);
    
    let pref_str = match preference {
        ComputePreference::Auto => "Auto (selección automática)",
        ComputePreference::Gpu => "GPU (CUDA/HIP/HIP-CPU)",
        ComputePreference::Cpu => "CPU puro",
        ComputePreference::Asm => "ASM SIMD optimizado",
        ComputePreference::LowPower => "Bajo consumo",
    };
    println!("⚙️  Preferencia: {}", pref_str);
    println!("------------------------");
    
    match engine.execute(&source) {
        Ok(result) => {
            println!("------------------------");
            println!("✅ Ejecución completada");
            println!("📊 Backend usado: {}", result.backend_used);
            println!("⏱️  Tiempo: {} μs", result.execution_time_us);
            println!("========================");
        }
        Err(e) => {
            eprintln!("❌ Error de ejecución: {}", e);
        }
    }
}

fn emit_file(file: &str, target: &str) {
    let source = match read_source(file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ {}", e);
            return;
        }
    };
    
    let result = match target {
        "--rust" | "-r" | "rust" => compile_to_rust(&source),
        "--c" | "-c" | "c" => compile_to_c(&source),
        "--asm" | "-a" | "asm" => compile_to_asm(&source),
        _ => {
            eprintln!("Target desconocido: {}", target);
            eprintln!("Usa: --rust, --c o --asm");
            return;
        }
    };
    
    match result {
        Ok(code) => println!("{}", code),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}
