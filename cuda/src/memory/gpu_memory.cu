/**
 * Super-C Runtime - GPU Memory Management
 * 
 * CUDA memory operations exposed via C ABI.
 * Memory is controlled by Rust arenas - no free malloc here.
 */

#include <cuda_runtime.h>
#include "super_c.h"

static int g_cuda_initialized = 0;

extern "C" {

int cuda_init(void) {
    if (g_cuda_initialized) {
        return SC_SUCCESS;
    }
    
    int device_count = 0;
    cudaError_t err = cudaGetDeviceCount(&device_count);
    
    if (err != cudaSuccess || device_count == 0) {
        return SC_ERROR_CUDA;
    }
    
    err = cudaSetDevice(0);
    if (err != cudaSuccess) {
        return SC_ERROR_CUDA;
    }
    
    g_cuda_initialized = 1;
    return SC_SUCCESS;
}

void cuda_shutdown(void) {
    if (!g_cuda_initialized) {
        return;
    }
    
    cudaDeviceReset();
    g_cuda_initialized = 0;
}

bool cuda_is_available(void) {
    int device_count = 0;
    cudaError_t err = cudaGetDeviceCount(&device_count);
    return (err == cudaSuccess && device_count > 0);
}

void* cuda_alloc(size_t size) {
    if (!g_cuda_initialized) {
        return NULL;
    }
    
    void* ptr = NULL;
    cudaError_t err = cudaMalloc(&ptr, size);
    
    if (err != cudaSuccess) {
        return NULL;
    }
    
    return ptr;
}

void cuda_free(void* ptr) {
    if (ptr != NULL) {
        cudaFree(ptr);
    }
}

int cuda_copy_to_device(void* dst, const void* src, size_t size) {
    cudaError_t err = cudaMemcpy(dst, src, size, cudaMemcpyHostToDevice);
    return (err == cudaSuccess) ? SC_SUCCESS : SC_ERROR_CUDA;
}

int cuda_copy_from_device(void* dst, const void* src, size_t size) {
    cudaError_t err = cudaMemcpy(dst, src, size, cudaMemcpyDeviceToHost);
    return (err == cudaSuccess) ? SC_SUCCESS : SC_ERROR_CUDA;
}

int cuda_launch_kernel(
    uint32_t kernel_id,
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
) {
    if (!g_cuda_initialized) {
        return SC_ERROR_INIT;
    }
    
    // TODO: Implement kernel dispatch based on kernel_id
    // For now, just return success
    (void)kernel_id;
    (void)data;
    (void)size;
    (void)output;
    (void)output_size;
    
    return SC_SUCCESS;
}

int cuda_sync(void) {
    cudaError_t err = cudaDeviceSynchronize();
    return (err == cudaSuccess) ? SC_SUCCESS : SC_ERROR_CUDA;
}

} // extern "C"
