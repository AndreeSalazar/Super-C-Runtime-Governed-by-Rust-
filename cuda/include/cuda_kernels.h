/**
 * Super-C Runtime - CUDA Kernels Header
 * 
 * Kernel declarations exposed via C ABI.
 */

#ifndef CUDA_KERNELS_H
#define CUDA_KERNELS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Kernel IDs */
#define KERNEL_VECTOR_ADD       0
#define KERNEL_MATRIX_MUL       1
#define KERNEL_REDUCE_SUM       2
#define KERNEL_TRANSFORM        3

/* Kernel launch configuration */
typedef struct {
    uint32_t block_size_x;
    uint32_t block_size_y;
    uint32_t block_size_z;
    uint32_t grid_size_x;
    uint32_t grid_size_y;
    uint32_t grid_size_z;
    size_t shared_mem_bytes;
} KernelConfig;

/* Default kernel configuration */
KernelConfig kernel_config_default(size_t data_size);

#ifdef __cplusplus
}
#endif

#endif /* CUDA_KERNELS_H */
