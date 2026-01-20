/**
 * Super-C Runtime - HIP Kernels Header
 * 
 * Portable GPU kernels that work on:
 * - AMD GPUs (native HIP)
 * - NVIDIA GPUs (HIP over CUDA)
 * - CPU (HIP-CPU fallback)
 */

#ifndef HIP_KERNELS_H
#define HIP_KERNELS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Backend Detection
 * ============================================================================ */

typedef enum {
    GPU_BACKEND_NONE = 0,
    GPU_BACKEND_CUDA = 1,      // NVIDIA CUDA
    GPU_BACKEND_HIP_AMD = 2,   // AMD HIP (native)
    GPU_BACKEND_HIP_NVIDIA = 3, // HIP over CUDA
    GPU_BACKEND_HIP_CPU = 4    // HIP-CPU (CPU fallback)
} GpuBackend;

/**
 * Get the active GPU backend
 */
GpuBackend gpu_get_backend(void);

/**
 * Get backend name as string
 */
const char* gpu_backend_name(GpuBackend backend);

/* ============================================================================
 * HIP Runtime API (C interface)
 * ============================================================================ */

/**
 * Initialize HIP runtime
 * @return 0 on success
 */
int hip_init(void);

/**
 * Shutdown HIP runtime
 */
void hip_shutdown(void);

/**
 * Check if HIP is available
 */
int hip_is_available(void);

/**
 * Get device count
 */
int hip_get_device_count(void);

/**
 * Allocate GPU memory
 */
void* hip_alloc(size_t size);

/**
 * Free GPU memory
 */
void hip_free(void* ptr);

/**
 * Copy host to device
 */
int hip_copy_to_device(void* dst, const void* src, size_t size);

/**
 * Copy device to host
 */
int hip_copy_from_device(void* dst, const void* src, size_t size);

/**
 * Synchronize device
 */
int hip_sync(void);

/* ============================================================================
 * Kernel IDs (shared with CUDA)
 * ============================================================================ */

#define KERNEL_VECTOR_ADD       0
#define KERNEL_MATRIX_MUL       1
#define KERNEL_REDUCE_SUM       2
#define KERNEL_TRANSFORM        3

/**
 * Launch a HIP kernel
 */
int hip_launch_kernel(
    uint32_t kernel_id,
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
);

#ifdef __cplusplus
}
#endif

#endif /* HIP_KERNELS_H */
