//! HIP FFI bindings
//!
//! Extern declarations for HIP functions (AMD GPU / HIP-CPU).
//! HIP → C → Rust (never direct HIP → Rust)

use std::ffi::c_void;

/// GPU Backend types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuBackend {
    None = 0,
    Cuda = 1,
    HipAmd = 2,
    HipNvidia = 3,
    HipCpu = 4,
}

/// GPU preference for initialization
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpuPreference {
    Performance = 0,
    PreferCuda = 1,
    PreferHip = 2,
    PreferCpu = 3,
}

// External HIP functions (implemented in hip/ layer, exposed via C)
extern "C" {
    /// Initialize HIP runtime
    pub fn hip_init() -> i32;

    /// Shutdown HIP runtime
    pub fn hip_shutdown();

    /// Check if HIP is available
    pub fn hip_is_available() -> i32;

    /// Get device count
    pub fn hip_get_device_count() -> i32;

    /// Allocate GPU memory
    pub fn hip_alloc(size: usize) -> *mut c_void;

    /// Free GPU memory
    pub fn hip_free(ptr: *mut c_void);

    /// Copy to device
    pub fn hip_copy_to_device(dst: *mut c_void, src: *const c_void, size: usize) -> i32;

    /// Copy from device
    pub fn hip_copy_from_device(dst: *mut c_void, src: *const c_void, size: usize) -> i32;

    /// Synchronize
    pub fn hip_sync() -> i32;

    /// Launch kernel
    pub fn hip_launch_kernel(
        kernel_id: u32,
        data: *const c_void,
        size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> i32;
}

// Unified GPU API
extern "C" {
    /// Initialize unified GPU subsystem
    pub fn gpu_init(pref: GpuPreference) -> i32;

    /// Shutdown unified GPU subsystem
    pub fn gpu_shutdown();

    /// Get active backend
    pub fn gpu_get_active_backend() -> GpuBackend;

    /// Get backend name
    pub fn gpu_get_backend_name() -> *const std::ffi::c_char;

    /// Check if GPU available
    pub fn gpu_is_available() -> bool;

    /// Get device count
    pub fn gpu_device_count() -> i32;

    /// Allocate GPU memory
    pub fn gpu_malloc(size: usize) -> *mut c_void;

    /// Free GPU memory
    pub fn gpu_free(ptr: *mut c_void);

    /// Copy host to device
    pub fn gpu_memcpy_h2d(dst: *mut c_void, src: *const c_void, size: usize) -> i32;

    /// Copy device to host
    pub fn gpu_memcpy_d2h(dst: *mut c_void, src: *const c_void, size: usize) -> i32;

    /// Synchronize
    pub fn gpu_sync() -> i32;

    /// Vector add
    pub fn gpu_vector_add_f32(a: *const f32, b: *const f32, c: *mut f32, n: usize) -> i32;

    /// Vector multiply
    pub fn gpu_vector_mul_f32(a: *const f32, b: *const f32, c: *mut f32, n: usize) -> i32;

    /// Vector scale
    pub fn gpu_vector_scale_f32(data: *mut f32, scale: f32, n: usize) -> i32;

    /// Reduce sum
    pub fn gpu_reduce_sum_f32(input: *const f32, output: *mut f32, n: usize) -> i32;
}

/// Safe wrapper for GPU initialization
pub fn init_gpu(preference: GpuPreference) -> Result<GpuBackend, i32> {
    let result = unsafe { gpu_init(preference) };
    if result == 0 {
        Ok(unsafe { gpu_get_active_backend() })
    } else {
        Err(result)
    }
}

/// Safe wrapper for GPU shutdown
pub fn shutdown_gpu() {
    unsafe { gpu_shutdown() };
}

/// Get the active GPU backend
pub fn get_backend() -> GpuBackend {
    unsafe { gpu_get_active_backend() }
}

/// Check if any GPU is available
pub fn is_gpu_available() -> bool {
    unsafe { gpu_is_available() }
}
