/**
 * Super-C Runtime - CPU/GPU Dispatcher
 * 
 * Handles dispatch decisions from Rust scheduler.
 */

#include "super_c.h"

typedef enum {
    DISPATCH_CPU = 0,
    DISPATCH_CPU_ASM = 1,
    DISPATCH_GPU = 2
} DispatchTarget;

int dispatch_execute(
    DispatchTarget target,
    const void* data,
    size_t size,
    void* output,
    size_t* output_size
) {
    switch (target) {
        case DISPATCH_CPU:
            return native_execute_cpu(data, size, output, output_size);
        
        case DISPATCH_CPU_ASM:
            return native_execute_cpu_asm(data, size, output, output_size);
        
        case DISPATCH_GPU:
            // Launch CUDA kernel
            return cuda_launch_kernel(0, data, size, output, output_size);
        
        default:
            return SC_ERROR_INVALID;
    }
}
