//! Safety Contracts
//!
//! Define and enforce safety invariants across the FFI boundary.
//! These contracts ensure that C/CUDA/ASM code respects Rust's safety guarantees.

/// Contract violation error
#[derive(Debug, Clone)]
pub struct ContractViolation {
    pub message: &'static str,
    pub location: &'static str,
}

/// Result type for contract-checked operations
pub type ContractResult<T> = Result<T, ContractViolation>;

/// Trait for types that can be safely passed across FFI
pub trait FfiSafe: Sized {
    /// Validate that the value is safe for FFI
    fn validate(&self) -> ContractResult<()>;
}

/// Marker trait for types that are safe to send to GPU
pub trait GpuSafe: FfiSafe {}

/// Marker trait for types that are safe for ASM hot paths
pub trait AsmSafe: FfiSafe {}

/// Pre-condition check macro helper
#[inline]
pub fn require(condition: bool, message: &'static str, location: &'static str) -> ContractResult<()> {
    if condition {
        Ok(())
    } else {
        Err(ContractViolation { message, location })
    }
}

/// Post-condition check macro helper
#[inline]
pub fn ensure(condition: bool, message: &'static str, location: &'static str) -> ContractResult<()> {
    require(condition, message, location)
}
