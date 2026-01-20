# Super-C Runtime
## *Governed by Rust • Accelerated by CUDA, HIP & ASM*

A high-performance native execution stack where **Rust governs**, **C/C++ executes**, and **CUDA/HIP/ASM accelerate**.

### GPU Support
- **NVIDIA** → CUDA (native)
- **AMD** → HIP (native)
- **CPU Fallback** → HIP-CPU (portable)

---

## Architecture

```
┌──────────────────────────────────────────────┐
│                APPLICATION                   │
│         (Engine / Tool / Runtime)            │
└──────────────────────────────────────────────┘
                     │
                     ▼
┌──────────────────────────────────────────────┐
│          RUST GOVERNOR (CORE BRAIN)          │
│                                              │
│  - Ownership / Lifetimes                     │
│  - Memory Arenas                             │
│  - Task Scheduler                            │
│  - Safety Contracts                          │
│  - GPU / CPU Dispatch                        │
└──────────────────────────────────────────────┘
          │                │
          ▼                ▼
┌─────────────────┐   ┌─────────────────┐
│   C / C++ CPU   │   │  CUDA/HIP GPU   │
│   (logic)       │   │   (kernels)     │
└─────────────────┘   └─────────────────┘
          │
          ▼
┌─────────────────┐
│   ASM (x86/ARM) │
│   (hot paths)   │
└─────────────────┘
```

---

## Project Structure

```
super-c-runtime/
├── rust/                   # Rust Governor (core brain)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── arena/          # Memory arenas
│   │   ├── scheduler/      # Task scheduler
│   │   ├── ffi/            # C/CUDA bindings
│   │   └── contracts/      # Safety contracts
│   ├── Cargo.toml
│   └── build.rs
│
├── native/                 # C/C++ executor layer
│   ├── include/            # Public headers (ABI)
│   ├── src/
│   │   ├── core/           # Core algorithms
│   │   ├── glue/           # Rust ↔ C glue
│   │   └── dispatch/       # CPU/GPU dispatch
│   └── CMakeLists.txt
│
├── cuda/                   # CUDA modules (NVIDIA GPU)
│   ├── include/
│   ├── src/
│   │   ├── kernels/        # CUDA kernels
│   │   └── memory/         # GPU memory management
│   └── CMakeLists.txt
│
├── hip/                    # HIP modules (AMD GPU / CPU fallback)
│   ├── include/
│   ├── src/
│   │   ├── kernels/        # HIP kernels (portable)
│   │   └── runtime/        # HIP runtime wrapper
│   └── CMakeLists.txt
│
├── external/               # External dependencies
│   └── hip-cpu/            # HIP-CPU (clone from AMD GitHub)
│
├── asm/                    # Assembly hot paths
│   ├── x86_64/             # x86-64 implementations
│   ├── aarch64/            # ARM64 implementations
│   └── include/            # C headers for ASM
│
├── dsl/                    # 🆕 SuperC DSL (sintaxis simplificada)
│   ├── src/
│   │   ├── lexer.rs        # Tokenizador
│   │   ├── parser.rs       # Parser
│   │   ├── ast.rs          # Abstract Syntax Tree
│   │   └── codegen.rs      # Generador de código
│   ├── examples/           # Ejemplos .sc
│   └── Cargo.toml
│
├── tests/                  # Integration tests
├── benches/                # Benchmarks
├── examples/               # Usage examples
└── docs/                   # Documentation
```

---

## Iron Rules

1. **CUDA and ASM NEVER talk to Rust directly**
   ```
   ASM  → C → Rust
   CUDA → C → Rust
   ```

2. **Rust governs, C executes, CUDA/ASM accelerate**

3. **No free malloc in CUDA/ASM** — memory controlled by Rust arenas

---

## Building

### Prerequisites

```bash
# Install HIP-CPU for CPU fallback (optional but recommended)
cd external
git clone https://github.com/ROCm-Developer-Tools/HIP-CPU.git hip-cpu
```

### Build Commands

```bash
# Build Rust core
cd rust && cargo build --release

# Build native C/C++ layer
cd native && cmake -B build && cmake --build build

# Build CUDA modules (requires NVIDIA CUDA toolkit)
cd cuda && cmake -B build && cmake --build build

# Build HIP modules (AMD GPU or HIP-CPU fallback)
cd hip && cmake -B build -DUSE_HIP_CPU=ON && cmake --build build
```

### Build Options

| Option | Description |
|--------|-------------|
| `-DUSE_HIP_CPU=ON` | Force HIP-CPU mode (no AMD GPU required) |
| `-DSUPER_C_HAS_CUDA=ON` | Enable CUDA support |
| `-DSUPER_C_HAS_HIP=ON` | Enable HIP support |

---

## GPU Backend Selection

The runtime automatically selects the best available backend:

```
1. CUDA (NVIDIA GPU) - if available
2. HIP (AMD GPU) - if available  
3. HIP-CPU (CPU fallback) - always available
```

You can force a specific backend via the Rust API:

```rust
use super_c_runtime::ffi::{init_gpu, GpuPreference};

// Auto-select best backend
init_gpu(GpuPreference::Performance)?;

// Force specific backend
init_gpu(GpuPreference::PreferCuda)?;  // NVIDIA
init_gpu(GpuPreference::PreferHip)?;   // AMD
init_gpu(GpuPreference::PreferCpu)?;   // HIP-CPU fallback
```

---

## 🚀 SuperC DSL - Motor de Cómputo Unificado

**[📖 Documentación Completa del DSL →](./dsl/README.md)**

Un lenguaje simplificado que ejecuta automáticamente en el mejor backend disponible:

```
.sc → CPU | GPU | ASM (automático)
```

### Ejemplo Rápido

```superc
data a: f32[1000]
data b: f32[1000]
data c: f32[1000]

parallel {
    for i = 0:1000 {
        c[i] = a[i] + b[i]
    }
}

seq {
    print(c[0])
}
```

### Ejecutar

```powershell
cd dsl
cargo build --release

# Ejecutar directamente (auto-selección de backend)
.\target\release\superc.exe run ejemplo.sc

# Forzar backend específico
.\target\release\superc.exe run ejemplo.sc --gpu   # GPU/HIP-CPU
.\target\release\superc.exe run ejemplo.sc --asm   # ASM SIMD
.\target\release\superc.exe run ejemplo.sc --cpu   # CPU puro

# Ver código generado
.\target\release\superc.exe emit ejemplo.sc --rust
.\target\release\superc.exe emit ejemplo.sc --c
.\target\release\superc.exe emit ejemplo.sc --asm
```

### Extensión VS Code

Instala la extensión para **coloreo de sintaxis** e **IntelliSense inteligente**:

```powershell
# Instalar desde VSIX
# En VS Code: Ctrl+Shift+P → "Extensions: Install from VSIX..."
# Seleccionar: dsl/vscode-extension/superc-lang/superc-lang-0.3.0.vsix
```

**Características:**
- Coloreo de sintaxis para `.sc`
- Autocompletado con `Ctrl+Space`
- Templates completos (`batch`, `vecadd`, `gpucompute`)
- Hover documentation
- Signature help para funciones

**[📖 Ver documentación completa del DSL →](./dsl/README.md)**

---

## License

MIT / Apache-2.0
