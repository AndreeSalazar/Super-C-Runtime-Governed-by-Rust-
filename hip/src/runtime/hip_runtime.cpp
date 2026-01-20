/**
 * Super-C Runtime - HIP Runtime Implementation
 * 
 * Portable GPU runtime that works with:
 * - AMD HIP (native)
 * - HIP-CPU (CPU fallback)
 */

#ifdef HIP_CPU_RUNTIME
    #include <hip/hip_runtime.h>
#else
    #include <hip/hip_runtime.h>
#endif

#include "hip_kernels.h"
#include "super_c.h"

static int g_hip_initialized = 0;
static GpuBackend g_backend = GPU_BACKEND_NONE;

extern "C" {

GpuBackend gpu_get_backend(void) {
    return g_backend;
}

const char* gpu_backend_name(GpuBackend backend) {
    switch (backend) {
        case GPU_BACKEND_CUDA:       return "CUDA (NVIDIA)";
        case GPU_BACKEND_HIP_AMD:    return "HIP (AMD)";
        case GPU_BACKEND_HIP_NVIDIA: return "HIP over CUDA";
        case GPU_BACKEND_HIP_CPU:    return "HIP-CPU (CPU fallback)";
        default:                     return "None";
    }
}

int hip_init(void) {
    if (g_hip_initialized) {
        return SC_SUCCESS;
    }
    
    hipError_t err = hipInit(0);
    if (err != hipSuccess) {
        return SC_ERROR_INIT;
    }
    
    int device_count = 0;
    err = hipGetDeviceCount(&device_count);
    
    if (err != hipSuccess || device_count == 0) {
        #ifdef HIP_CPU_RUNTIME
            g_backend = GPU_BACKEND_HIP_CPU;
            g_hip_initialized = 1;
            return SC_SUCCESS;
        #else
            return SC_ERROR_INIT;
        #endif
    }
    
    err = hipSetDevice(0);
    if (err != hipSuccess) {
        return SC_ERROR_INIT;
    }
    
    // Detect backend
    #if defined(HIP_CPU_RUNTIME) || defined(__HIP_PLATFORM_CPU__)
        g_backend = GPU_BACKEND_HIP_CPU;
    #elif defined(__HIP_PLATFORM_AMD__) || defined(__HIP_PLATFORM_HCC__)
        g_backend = GPU_BACKEND_HIP_AMD;
    #elif defined(__HIP_PLATFORM_NVIDIA__) || defined(__HIP_PLATFORM_NVCC__)
        g_backend = GPU_BACKEND_HIP_NVIDIA;
    #else
        g_backend = GPU_BACKEND_HIP_AMD; // Default assumption
    #endif
    
    g_hip_initialized = 1;
    return SC_SUCCESS;
}

void hip_shutdown(void) {
    if (!g_hip_initialized) {
        return;
    }
    
    hipDeviceReset();
    g_hip_initialized = 0;
    g_backend = GPU_BACKEND_NONE;
}

int hip_is_available(void) {
    if (g_hip_initialized) {
        return 1;
    }
    
    int device_count = 0;
    hipError_t err = hipGetDeviceCount(&device_count);
    
    #ifdef HIP_CPU_RUNTIME
        return 1; // HIP-CPU always available
    #else
        return (err == hipSuccess && device_count > 0) ? 1 : 0;
    #endif
}

int hip_get_device_count(void) {
    int count = 0;
    hipGetDeviceCount(&count);
    
    #ifdef HIP_CPU_RUNTIME
        if (count == 0) count = 1; // HIP-CPU simulates 1 device
    #endif
    
    return count;
}

void* hip_alloc(size_t size) {
    if (!g_hip_initialized) {
        return nullptr;
    }
    
    void* ptr = nullptr;
    hipError_t err = hipMalloc(&ptr, size);
    
    return (err == hipSuccess) ? ptr : nullptr;
}

void hip_free(void* ptr) {
    if (ptr != nullptr) {
        hipFree(ptr);
    }
}

int hip_copy_to_device(void* dst, const void* src, size_t size) {
    hipError_t err = hipMemcpy(dst, src, size, hipMemcpyHostToDevice);
    return (err == hipSuccess) ? SC_SUCCESS : SC_ERROR_MEMORY;
}

int hip_copy_from_device(void* dst, const void* src, size_t size) {
    hipError_t err = hipMemcpy(dst, src, size, hipMemcpyDeviceToHost);
    return (err == hipSuccess) ? SC_SUCCESS : SC_ERROR_MEMORY;
}

int hip_sync(void) {
    hipError_t err = hipDeviceSynchronize();
    return (err == hipSuccess) ? SC_SUCCESS : SC_ERROR_CUDA;
}

int hip_launch_kernel(
    uint32_t kernel_id,
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
) {
    if (!g_hip_initialized) {
        return SC_ERROR_INIT;
    }
    
    // TODO: Implement kernel dispatch
    (void)kernel_id;
    (void)data;
    (void)size;
    (void)output;
    (void)output_size;
    
    return SC_SUCCESS;
}

} // extern "C"
