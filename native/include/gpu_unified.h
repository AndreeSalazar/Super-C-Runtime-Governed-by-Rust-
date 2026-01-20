/**
 * Super-C Runtime - Unified GPU Abstraction Layer
 * 
 * Single API that works across:
 * - CUDA (NVIDIA)
 * - HIP (AMD)
 * - HIP-CPU (CPU fallback)
 * 
 * The dispatcher automatically selects the best available backend.
 */

#ifndef GPU_UNIFIED_H
#define GPU_UNIFIED_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Backend Types
 * ============================================================================ */

typedef enum {
    GPU_BACKEND_NONE = 0,
    GPU_BACKEND_CUDA = 1,       // NVIDIA CUDA
    GPU_BACKEND_HIP_AMD = 2,    // AMD HIP (native)
    GPU_BACKEND_HIP_NVIDIA = 3, // HIP over CUDA
    GPU_BACKEND_HIP_CPU = 4     // HIP-CPU (CPU fallback)
} GpuBackend;

typedef enum {
    GPU_PREFER_PERFORMANCE = 0, // Prefer fastest backend
    GPU_PREFER_CUDA = 1,        // Prefer CUDA if available
    GPU_PREFER_HIP = 2,         // Prefer HIP if available
    GPU_PREFER_CPU = 3          // Force CPU execution
} GpuPreference;

/* ============================================================================
 * Unified GPU API
 * ============================================================================ */

/**
 * Initialize GPU subsystem with preference
 * @param pref Backend preference
 * @return 0 on success
 */
int gpu_init(GpuPreference pref);

/**
 * Shutdown GPU subsystem
 */
void gpu_shutdown(void);

/**
 * Get active backend
 */
GpuBackend gpu_get_active_backend(void);

/**
 * Get backend name
 */
const char* gpu_get_backend_name(void);

/**
 * Check if GPU is available
 */
bool gpu_is_available(void);

/**
 * Get device count
 */
int gpu_device_count(void);

/* ============================================================================
 * Memory Operations
 * ============================================================================ */

/**
 * Allocate device memory
 */
void* gpu_malloc(size_t size);

/**
 * Free device memory
 */
void gpu_free(void* ptr);

/**
 * Copy host to device
 */
int gpu_memcpy_h2d(void* dst, const void* src, size_t size);

/**
 * Copy device to host
 */
int gpu_memcpy_d2h(void* dst, const void* src, size_t size);

/**
 * Copy device to device
 */
int gpu_memcpy_d2d(void* dst, const void* src, size_t size);

/**
 * Set device memory
 */
int gpu_memset(void* ptr, int value, size_t size);

/**
 * Synchronize device
 */
int gpu_sync(void);

/* ============================================================================
 * Kernel Execution
 * ============================================================================ */

/**
 * Launch kernel by ID
 */
int gpu_launch_kernel(
    uint32_t kernel_id,
    const void* input,
    size_t input_size,
    void* output,
    size_t* output_size
);

/* ============================================================================
 * Vector Operations (high-level API)
 * ============================================================================ */

int gpu_vector_add_f32(const float* a, const float* b, float* c, size_t n);
int gpu_vector_mul_f32(const float* a, const float* b, float* c, size_t n);
int gpu_vector_scale_f32(float* data, float scale, size_t n);
int gpu_reduce_sum_f32(const float* input, float* output, size_t n);

#ifdef __cplusplus
}
#endif

#endif /* GPU_UNIFIED_H */
