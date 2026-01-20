//! Task definitions and types

use std::ffi::c_void;

/// Task priority levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Execution target for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskTarget {
    /// CPU execution (C code)
    Cpu,
    /// CPU with ASM hot paths
    CpuAsm,
    /// GPU execution (CUDA)
    Gpu,
    /// Auto-select based on workload
    Auto,
}

/// A unit of work to be scheduled
pub struct Task {
    pub id: u64,
    pub priority: TaskPriority,
    pub target: TaskTarget,
    pub data: *const c_void,
    pub data_size: usize,
}

impl Task {
    pub fn new(id: u64, target: TaskTarget) -> Self {
        Self {
            id,
            priority: TaskPriority::Normal,
            target,
            data: std::ptr::null(),
            data_size: 0,
        }
    }
}

/// Handle to a submitted task
#[derive(Debug, Clone, Copy)]
pub struct TaskHandle {
    pub id: u64,
}

/// Result of task execution
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskResult {
    Success,
    Failed,
    Cancelled,
    Pending,
}
