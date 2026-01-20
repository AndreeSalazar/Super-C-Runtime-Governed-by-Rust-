/**
 * Super-C Runtime - Native C API
 * 
 * Public ABI for Rust ↔ C interop.
 * All CUDA and ASM calls go through this layer.
 */

#ifndef SUPER_C_H
#define SUPER_C_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Error Codes
 * ============================================================================ */

#define SC_SUCCESS          0
#define SC_ERROR_INIT      -1
#define SC_ERROR_MEMORY    -2
#define SC_ERROR_INVALID   -3
#define SC_ERROR_CUDA      -4
#define SC_ERROR_ASM       -5

/* ============================================================================
 * Native Runtime API
 * ============================================================================ */

/**
 * Initialize native runtime
 * @return SC_SUCCESS on success, error code otherwise
 */
int native_init(void);

/**
 * Shutdown native runtime
 */
void native_shutdown(void);

/**
 * Execute CPU workload
 * @param data Input data pointer
 * @param size Input data size
 * @param output Output buffer pointer
 * @param output_size Output size (in/out)
 * @return SC_SUCCESS on success
 */
int native_execute_cpu(
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
);

/**
 * Execute CPU workload with ASM hot paths
 * @param data Input data pointer
 * @param size Input data size
 * @param output Output buffer pointer
 * @param output_size Output size (in/out)
 * @return SC_SUCCESS on success
 */
int native_execute_cpu_asm(
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
);

/* ============================================================================
 * CUDA API (exposed via C)
 * ============================================================================ */

/**
 * Initialize CUDA runtime
 * @return SC_SUCCESS on success
 */
int cuda_init(void);

/**
 * Shutdown CUDA runtime
 */
void cuda_shutdown(void);

/**
 * Check if CUDA is available
 * @return true if CUDA is available
 */
bool cuda_is_available(void);

/**
 * Allocate GPU memory
 * @param size Bytes to allocate
 * @return Pointer to GPU memory, NULL on failure
 */
void* cuda_alloc(size_t size);

/**
 * Free GPU memory
 * @param ptr Pointer to GPU memory
 */
void cuda_free(void* ptr);

/**
 * Copy data to GPU
 * @param dst GPU destination
 * @param src Host source
 * @param size Bytes to copy
 * @return SC_SUCCESS on success
 */
int cuda_copy_to_device(void* dst, const void* src, size_t size);

/**
 * Copy data from GPU
 * @param dst Host destination
 * @param src GPU source
 * @param size Bytes to copy
 * @return SC_SUCCESS on success
 */
int cuda_copy_from_device(void* dst, const void* src, size_t size);

/**
 * Launch a CUDA kernel
 * @param kernel_id Kernel identifier
 * @param data Input data
 * @param size Input size
 * @param output Output buffer
 * @param output_size Output size (in/out)
 * @return SC_SUCCESS on success
 */
int cuda_launch_kernel(
    uint32_t kernel_id,
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
);

/**
 * Synchronize GPU
 * @return SC_SUCCESS on success
 */
int cuda_sync(void);

#ifdef __cplusplus
}
#endif

#endif /* SUPER_C_H */
