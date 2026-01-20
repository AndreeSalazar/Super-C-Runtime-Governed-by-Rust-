//! CUDA FFI bindings
//!
//! Extern declarations for CUDA functions.
//! CUDA → C → Rust (never direct CUDA → Rust)

use std::ffi::c_void;

// External CUDA functions (implemented in cuda/ layer, exposed via C)
extern "C" {
    /// Initialize CUDA runtime
    pub fn cuda_init() -> i32;

    /// Shutdown CUDA runtime
    pub fn cuda_shutdown();

    /// Check if CUDA is available
    pub fn cuda_is_available() -> bool;

    /// Allocate GPU memory (returns handle)
    pub fn cuda_alloc(size: usize) -> *mut c_void;

    /// Free GPU memory
    pub fn cuda_free(ptr: *mut c_void);

    /// Copy data to GPU
    pub fn cuda_copy_to_device(
        dst: *mut c_void,
        src: *const c_void,
        size: usize,
    ) -> i32;

    /// Copy data from GPU
    pub fn cuda_copy_from_device(
        dst: *mut c_void,
        src: *const c_void,
        size: usize,
    ) -> i32;

    /// Launch a kernel
    pub fn cuda_launch_kernel(
        kernel_id: u32,
        data: *const c_void,
        size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> i32;

    /// Synchronize GPU
    pub fn cuda_sync() -> i32;
}

/// Safe wrapper for CUDA initialization
pub fn init_cuda() -> Result<(), i32> {
    let result = unsafe { cuda_init() };
    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}

/// Safe wrapper for CUDA shutdown
pub fn shutdown_cuda() {
    unsafe { cuda_shutdown() };
}

/// Check CUDA availability
pub fn is_cuda_available() -> bool {
    unsafe { cuda_is_available() }
}
