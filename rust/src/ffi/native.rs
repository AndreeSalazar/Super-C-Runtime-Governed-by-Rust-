//! Native C/C++ FFI bindings
//!
//! Extern declarations for C functions.
//! ASM → C → Rust (never direct ASM → Rust)

use std::ffi::c_void;

/// External C functions (implemented in native/ layer)
extern "C" {
    /// Initialize native runtime
    pub fn native_init() -> i32;

    /// Shutdown native runtime
    pub fn native_shutdown();

    /// Execute CPU workload
    pub fn native_execute_cpu(
        data: *const c_void,
        size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> i32;

    /// Execute CPU workload with ASM hot paths
    pub fn native_execute_cpu_asm(
        data: *const c_void,
        size: usize,
        output: *mut c_void,
        output_size: *mut usize,
    ) -> i32;
}

/// Safe wrapper for native initialization
pub fn init_native() -> Result<(), i32> {
    let result = unsafe { native_init() };
    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}

/// Safe wrapper for native shutdown
pub fn shutdown_native() {
    unsafe { native_shutdown() };
}
