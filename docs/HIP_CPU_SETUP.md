# HIP-CPU Setup Guide

## What is HIP-CPU?

HIP-CPU is AMD's implementation of HIP that runs on CPU instead of GPU. This allows:

- **Development** without AMD GPU hardware
- **Testing** on any machine
- **Fallback** when no GPU is available
- **Portability** across all platforms

## Installation

### Step 1: Clone HIP-CPU

```bash
cd external
git clone https://github.com/ROCm-Developer-Tools/HIP-CPU.git hip-cpu
```

### Step 2: Set Environment Variable (Optional)

```bash
# Linux/macOS
export HIP_CPU_PATH=/path/to/super-c-runtime/external/hip-cpu

# Windows (PowerShell)
$env:HIP_CPU_PATH = "C:\path\to\super-c-runtime\external\hip-cpu"
```

### Step 3: Install TBB (Recommended)

HIP-CPU uses Intel TBB for parallel execution:

```bash
# Windows (vcpkg)
vcpkg install tbb:x64-windows

# Linux (Ubuntu/Debian)
sudo apt install libtbb-dev

# Linux (Fedora)
sudo dnf install tbb-devel

# macOS
brew install tbb
```

## Building with HIP-CPU

```bash
cd hip
cmake -B build -DUSE_HIP_CPU=ON
cmake --build build --config Release
```

## Verifying Installation

After building, you can verify HIP-CPU is working:

```cpp
#include "hip_kernels.h"
#include <stdio.h>

int main() {
    if (hip_init() == 0) {
        printf("Backend: %s\n", gpu_backend_name(gpu_get_backend()));
        printf("Devices: %d\n", hip_get_device_count());
        hip_shutdown();
    }
    return 0;
}
```

Expected output:
```
Backend: HIP-CPU (CPU fallback)
Devices: 1
```

## Performance Notes

HIP-CPU is **not** as fast as real GPU execution, but it provides:

| Feature | HIP-CPU | Real GPU |
|---------|---------|----------|
| Correctness | ✅ Same | ✅ Same |
| Parallelism | Multi-threaded | Massively parallel |
| Memory | System RAM | VRAM |
| Use case | Dev/Test | Production |

## Troubleshooting

### "HIP not found" error

Ensure `HIP_CPU_PATH` is set or the `hip-cpu` folder exists in `external/`.

### TBB not found

Install TBB or build without it (slower but works):
```bash
cmake -B build -DUSE_HIP_CPU=ON -DHIP_CPU_USE_TBB=OFF
```

### Compilation errors

Ensure you have a C++17 compatible compiler:
- GCC 7+
- Clang 5+
- MSVC 2017+

## Architecture Flow

```
Rust Governor
     │
     ▼
┌─────────────────────────────────────┐
│         Unified GPU API             │
│         (gpu_unified.h)             │
└─────────────────────────────────────┘
     │
     ├──► CUDA (NVIDIA) ──► GPU
     │
     ├──► HIP (AMD) ──► AMD GPU
     │
     └──► HIP-CPU ──► CPU (multi-threaded)
```

All three backends use the **same kernel code** — write once, run anywhere.
