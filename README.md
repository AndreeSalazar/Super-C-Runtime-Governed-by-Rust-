# Super-C Runtime
## *Governed by Rust • Accelerated by CUDA & ASM*

A high-performance native execution stack where **Rust governs**, **C/C++ executes**, and **CUDA/ASM accelerate**.

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
│   C / C++ CPU   │   │   CUDA MODULES  │
│   (logic)       │   │   (GPU kernels) │
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
├── cuda/                   # CUDA modules (GPU kernels)
│   ├── include/
│   ├── src/
│   │   ├── kernels/        # CUDA kernels
│   │   └── memory/         # GPU memory management
│   └── CMakeLists.txt
│
├── asm/                    # Assembly hot paths
│   ├── x86_64/             # x86-64 implementations
│   ├── aarch64/            # ARM64 implementations
│   └── include/            # C headers for ASM
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

```bash
# Build Rust core
cd rust && cargo build --release

# Build native C/C++ layer
cd native && cmake -B build && cmake --build build

# Build CUDA modules (requires CUDA toolkit)
cd cuda && cmake -B build && cmake --build build
```

---

## License

MIT / Apache-2.0
