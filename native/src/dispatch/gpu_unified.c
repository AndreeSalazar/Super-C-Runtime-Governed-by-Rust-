/**
 * Super-C Runtime - Unified GPU Dispatcher
 * 
 * Automatically selects and dispatches to the best available GPU backend.
 */

#include "gpu_unified.h"
#include "super_c.h"
#include <string.h>

// Forward declarations for backend-specific functions
#ifdef SUPER_C_HAS_CUDA
extern int cuda_init(void);
extern void cuda_shutdown(void);
extern bool cuda_is_available(void);
extern void* cuda_alloc(size_t size);
extern void cuda_free(void* ptr);
extern int cuda_copy_to_device(void* dst, const void* src, size_t size);
extern int cuda_copy_from_device(void* dst, const void* src, size_t size);
extern int cuda_sync(void);
extern int cuda_launch_kernel(uint32_t kernel_id, const void* data, size_t size, void* output, size_t* output_size);
#endif

#ifdef SUPER_C_HAS_HIP
extern int hip_init(void);
extern void hip_shutdown(void);
extern int hip_is_available(void);
extern int hip_get_device_count(void);
extern void* hip_alloc(size_t size);
extern void hip_free(void* ptr);
extern int hip_copy_to_device(void* dst, const void* src, size_t size);
extern int hip_copy_from_device(void* dst, const void* src, size_t size);
extern int hip_sync(void);
extern int hip_launch_kernel(uint32_t kernel_id, const void* data, size_t size, void* output, size_t* output_size);
extern int hip_vector_add(const float* a, const float* b, float* c, size_t n);
extern int hip_vector_mul(const float* a, const float* b, float* c, size_t n);
extern int hip_vector_scale(float* data, float scale, size_t n);
extern int hip_reduce_sum(const float* input, float* output, size_t n);
#endif

static GpuBackend g_active_backend = GPU_BACKEND_NONE;
static int g_initialized = 0;

int gpu_init(GpuPreference pref) {
    if (g_initialized) {
        return SC_SUCCESS;
    }
    
    // Try backends based on preference
    switch (pref) {
        case GPU_PREFER_CUDA:
            #ifdef SUPER_C_HAS_CUDA
            if (cuda_is_available() && cuda_init() == SC_SUCCESS) {
                g_active_backend = GPU_BACKEND_CUDA;
                g_initialized = 1;
                return SC_SUCCESS;
            }
            #endif
            // Fall through to HIP
            
        case GPU_PREFER_HIP:
            #ifdef SUPER_C_HAS_HIP
            if (hip_is_available() && hip_init() == SC_SUCCESS) {
                // Backend type is set by hip_init
                g_active_backend = GPU_BACKEND_HIP_AMD; // Will be corrected by HIP
                g_initialized = 1;
                return SC_SUCCESS;
            }
            #endif
            break;
            
        case GPU_PREFER_CPU:
            #ifdef SUPER_C_HAS_HIP
            // Force HIP-CPU mode
            if (hip_init() == SC_SUCCESS) {
                g_active_backend = GPU_BACKEND_HIP_CPU;
                g_initialized = 1;
                return SC_SUCCESS;
            }
            #endif
            break;
            
        case GPU_PREFER_PERFORMANCE:
        default:
            // Try CUDA first, then HIP
            #ifdef SUPER_C_HAS_CUDA
            if (cuda_is_available() && cuda_init() == SC_SUCCESS) {
                g_active_backend = GPU_BACKEND_CUDA;
                g_initialized = 1;
                return SC_SUCCESS;
            }
            #endif
            
            #ifdef SUPER_C_HAS_HIP
            if (hip_init() == SC_SUCCESS) {
                g_active_backend = GPU_BACKEND_HIP_AMD;
                g_initialized = 1;
                return SC_SUCCESS;
            }
            #endif
            break;
    }
    
    return SC_ERROR_INIT;
}

void gpu_shutdown(void) {
    if (!g_initialized) {
        return;
    }
    
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            cuda_shutdown();
            break;
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            hip_shutdown();
            break;
        #endif
        
        default:
            break;
    }
    
    g_active_backend = GPU_BACKEND_NONE;
    g_initialized = 0;
}

GpuBackend gpu_get_active_backend(void) {
    return g_active_backend;
}

const char* gpu_get_backend_name(void) {
    switch (g_active_backend) {
        case GPU_BACKEND_CUDA:       return "CUDA (NVIDIA)";
        case GPU_BACKEND_HIP_AMD:    return "HIP (AMD)";
        case GPU_BACKEND_HIP_NVIDIA: return "HIP over CUDA";
        case GPU_BACKEND_HIP_CPU:    return "HIP-CPU";
        default:                     return "None";
    }
}

bool gpu_is_available(void) {
    #ifdef SUPER_C_HAS_CUDA
    if (cuda_is_available()) return true;
    #endif
    
    #ifdef SUPER_C_HAS_HIP
    if (hip_is_available()) return true;
    #endif
    
    return false;
}

int gpu_device_count(void) {
    #ifdef SUPER_C_HAS_HIP
    return hip_get_device_count();
    #else
    return 0;
    #endif
}

void* gpu_malloc(size_t size) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            return cuda_alloc(size);
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            return hip_alloc(size);
        #endif
        
        default:
            return NULL;
    }
}

void gpu_free(void* ptr) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            cuda_free(ptr);
            break;
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            hip_free(ptr);
            break;
        #endif
        
        default:
            break;
    }
}

int gpu_memcpy_h2d(void* dst, const void* src, size_t size) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            return cuda_copy_to_device(dst, src, size);
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            return hip_copy_to_device(dst, src, size);
        #endif
        
        default:
            return SC_ERROR_INIT;
    }
}

int gpu_memcpy_d2h(void* dst, const void* src, size_t size) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            return cuda_copy_from_device(dst, src, size);
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            return hip_copy_from_device(dst, src, size);
        #endif
        
        default:
            return SC_ERROR_INIT;
    }
}

int gpu_memcpy_d2d(void* dst, const void* src, size_t size) {
    // For now, use same path as h2d (works for HIP-CPU)
    return gpu_memcpy_h2d(dst, src, size);
}

int gpu_memset(void* ptr, int value, size_t size) {
    // TODO: Implement proper GPU memset
    (void)ptr;
    (void)value;
    (void)size;
    return SC_SUCCESS;
}

int gpu_sync(void) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            return cuda_sync();
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            return hip_sync();
        #endif
        
        default:
            return SC_SUCCESS;
    }
}

int gpu_launch_kernel(
    uint32_t kernel_id,
    const void* input,
    size_t input_size,
    void* output,
    size_t* output_size
) {
    switch (g_active_backend) {
        #ifdef SUPER_C_HAS_CUDA
        case GPU_BACKEND_CUDA:
            return cuda_launch_kernel(kernel_id, input, input_size, output, output_size);
        #endif
        
        #ifdef SUPER_C_HAS_HIP
        case GPU_BACKEND_HIP_AMD:
        case GPU_BACKEND_HIP_NVIDIA:
        case GPU_BACKEND_HIP_CPU:
            return hip_launch_kernel(kernel_id, input, input_size, output, output_size);
        #endif
        
        default:
            return SC_ERROR_INIT;
    }
}

// High-level vector operations
int gpu_vector_add_f32(const float* a, const float* b, float* c, size_t n) {
    #ifdef SUPER_C_HAS_HIP
    if (g_active_backend == GPU_BACKEND_HIP_AMD ||
        g_active_backend == GPU_BACKEND_HIP_NVIDIA ||
        g_active_backend == GPU_BACKEND_HIP_CPU) {
        return hip_vector_add(a, b, c, n);
    }
    #endif
    
    // CPU fallback
    for (size_t i = 0; i < n; i++) {
        c[i] = a[i] + b[i];
    }
    return SC_SUCCESS;
}

int gpu_vector_mul_f32(const float* a, const float* b, float* c, size_t n) {
    #ifdef SUPER_C_HAS_HIP
    if (g_active_backend == GPU_BACKEND_HIP_AMD ||
        g_active_backend == GPU_BACKEND_HIP_NVIDIA ||
        g_active_backend == GPU_BACKEND_HIP_CPU) {
        return hip_vector_mul(a, b, c, n);
    }
    #endif
    
    for (size_t i = 0; i < n; i++) {
        c[i] = a[i] * b[i];
    }
    return SC_SUCCESS;
}

int gpu_vector_scale_f32(float* data, float scale, size_t n) {
    #ifdef SUPER_C_HAS_HIP
    if (g_active_backend == GPU_BACKEND_HIP_AMD ||
        g_active_backend == GPU_BACKEND_HIP_NVIDIA ||
        g_active_backend == GPU_BACKEND_HIP_CPU) {
        return hip_vector_scale(data, scale, n);
    }
    #endif
    
    for (size_t i = 0; i < n; i++) {
        data[i] *= scale;
    }
    return SC_SUCCESS;
}

int gpu_reduce_sum_f32(const float* input, float* output, size_t n) {
    #ifdef SUPER_C_HAS_HIP
    if (g_active_backend == GPU_BACKEND_HIP_AMD ||
        g_active_backend == GPU_BACKEND_HIP_NVIDIA ||
        g_active_backend == GPU_BACKEND_HIP_CPU) {
        return hip_reduce_sum(input, output, n);
    }
    #endif
    
    float sum = 0.0f;
    for (size_t i = 0; i < n; i++) {
        sum += input[i];
    }
    *output = sum;
    return SC_SUCCESS;
}
