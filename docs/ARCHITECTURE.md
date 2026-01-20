# Super-C Runtime Architecture

## Overview

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

## Iron Rules

### 1. CUDA and ASM NEVER talk to Rust directly

```
ASM  → C → Rust
CUDA → C → Rust
```

**Reasons:**
- ABI stability
- Debuggable
- Replaceable
- Portable

### 2. Technology Roles

| Layer | Role | Responsibility |
|-------|------|----------------|
| **Rust** | Governor | Decides WHAT, WHEN, HOW MUCH memory |
| **C/C++** | Executor | Implements algorithms, calls ASM, launches CUDA |
| **CUDA** | GPU Accelerator | Massive parallel workloads only |
| **ASM** | Hot Path Optimizer | Last 1-5% performance critical code |

### 3. Memory Control

- CUDA does **no free malloc**
- ASM does **not touch heap**
- All memory flows through Rust arenas

## Execution Flow

```
Rust:
  validates input
  allocates arena
  decides CPU or GPU
    ↓
C:
  if GPU → launches CUDA
  if CPU → calls ASM
    ↓
CUDA / ASM:
  executes
  returns data
    ↓
Rust:
  validates output
  synchronizes
  frees
```

## When to Use Each Technology

### CUDA (GPU)
✅ Use for:
- Massive simulations
- Custom ML
- Physics
- Graphics
- Large batch processing

❌ Don't use for:
- Business logic
- Control flow
- Small workloads

### ASM (Hot Paths)
✅ Use for:
- Custom memcpy
- Hashing
- Compression
- Crypto
- Extreme SIMD

❌ Don't use if:
- You can't measure the improvement
- C/C++ intrinsics are sufficient
