/**
 * Super-C Runtime - HIP Vector Operations
 * 
 * Portable kernels that compile for AMD, NVIDIA, or CPU
 */

#ifdef HIP_CPU_RUNTIME
    #include <hip/hip_runtime.h>
#else
    #include <hip/hip_runtime.h>
#endif

#include "hip_kernels.h"

// ============================================================================
// HIP Kernels (portable across AMD/NVIDIA/CPU)
// ============================================================================

__global__ void vector_add_kernel(const float* a, const float* b, float* c, size_t n) {
    size_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        c[idx] = a[idx] + b[idx];
    }
}

__global__ void vector_scale_kernel(float* data, float scale, size_t n) {
    size_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        data[idx] *= scale;
    }
}

__global__ void vector_mul_kernel(const float* a, const float* b, float* c, size_t n) {
    size_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        c[idx] = a[idx] * b[idx];
    }
}

__global__ void reduce_sum_kernel(const float* input, float* output, size_t n) {
    extern __shared__ float sdata[];
    
    size_t tid = threadIdx.x;
    size_t idx = blockIdx.x * blockDim.x + threadIdx.x;
    
    sdata[tid] = (idx < n) ? input[idx] : 0.0f;
    __syncthreads();
    
    for (size_t s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            sdata[tid] += sdata[tid + s];
        }
        __syncthreads();
    }
    
    if (tid == 0) {
        atomicAdd(output, sdata[0]);
    }
}

// ============================================================================
// Kernel Launch Wrappers
// ============================================================================

extern "C" {

int hip_vector_add(const float* a, const float* b, float* c, size_t n) {
    const int block_size = 256;
    const int grid_size = (n + block_size - 1) / block_size;
    
    hipLaunchKernelGGL(vector_add_kernel, 
        dim3(grid_size), dim3(block_size), 0, 0,
        a, b, c, n);
    
    return (hipGetLastError() == hipSuccess) ? 0 : -1;
}

int hip_vector_scale(float* data, float scale, size_t n) {
    const int block_size = 256;
    const int grid_size = (n + block_size - 1) / block_size;
    
    hipLaunchKernelGGL(vector_scale_kernel,
        dim3(grid_size), dim3(block_size), 0, 0,
        data, scale, n);
    
    return (hipGetLastError() == hipSuccess) ? 0 : -1;
}

int hip_vector_mul(const float* a, const float* b, float* c, size_t n) {
    const int block_size = 256;
    const int grid_size = (n + block_size - 1) / block_size;
    
    hipLaunchKernelGGL(vector_mul_kernel,
        dim3(grid_size), dim3(block_size), 0, 0,
        a, b, c, n);
    
    return (hipGetLastError() == hipSuccess) ? 0 : -1;
}

int hip_reduce_sum(const float* input, float* output, size_t n) {
    const int block_size = 256;
    const int grid_size = (n + block_size - 1) / block_size;
    const size_t shared_mem = block_size * sizeof(float);
    
    hipLaunchKernelGGL(reduce_sum_kernel,
        dim3(grid_size), dim3(block_size), shared_mem, 0,
        input, output, n);
    
    return (hipGetLastError() == hipSuccess) ? 0 : -1;
}

} // extern "C"
