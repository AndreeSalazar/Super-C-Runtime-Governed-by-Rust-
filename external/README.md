# External Dependencies

This folder contains external dependencies that need to be installed manually.

## HIP-CPU (AMD)

HIP-CPU allows running HIP code on CPU without an AMD GPU. This is useful for:
- Development on machines without AMD GPUs
- Testing and debugging
- Fallback execution

### Installation

```bash
# Clone HIP-CPU from AMD's GitHub
git clone https://github.com/ROCm-Developer-Tools/HIP-CPU.git hip-cpu

# Or set environment variable to your HIP-CPU installation
export HIP_CPU_PATH=/path/to/hip-cpu
```

### Repository Structure After Installation

```
external/
└── hip-cpu/
    ├── include/
    │   └── hip/
    │       └── hip_runtime.h
    └── ...
```

## Optional: TBB (Threading Building Blocks)

HIP-CPU can use Intel TBB for better parallel performance:

```bash
# Windows (vcpkg)
vcpkg install tbb

# Linux
sudo apt install libtbb-dev

# macOS
brew install tbb
```
