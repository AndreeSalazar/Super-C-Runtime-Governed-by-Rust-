//! FFI Bindings
//!
//! Safe wrappers for C/CUDA/ASM interop.
//! All external calls go through this module.

mod native;
mod cuda;
mod hip;

pub use native::*;
pub use cuda::*;
pub use hip::*;

use std::ffi::c_void;

/// Opaque handle to native resources
#[repr(C)]
pub struct NativeHandle {
    ptr: *mut c_void,
}

impl NativeHandle {
    pub fn null() -> Self {
        Self { ptr: std::ptr::null_mut() }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

/// Dispatch target for execution
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchTarget {
    /// Execute on CPU (C code)
    Cpu = 0,
    /// Execute on CPU with ASM hot paths
    CpuAsm = 1,
    /// Execute on GPU (CUDA)
    Gpu = 2,
}

/// Result from native execution
#[repr(C)]
pub struct NativeResult {
    pub success: bool,
    pub error_code: i32,
    pub data: *mut c_void,
    pub data_size: usize,
}
