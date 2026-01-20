//! Dispatch logic for CPU/GPU execution

use super::task::{Task, TaskTarget};
use crate::ffi::DispatchTarget;

/// Determine the best execution target for a task
pub fn select_target(task: &Task, gpu_available: bool, asm_enabled: bool) -> DispatchTarget {
    match task.target {
        TaskTarget::Cpu => DispatchTarget::Cpu,
        TaskTarget::CpuAsm => {
            if asm_enabled {
                DispatchTarget::CpuAsm
            } else {
                DispatchTarget::Cpu
            }
        }
        TaskTarget::Gpu => {
            if gpu_available {
                DispatchTarget::Gpu
            } else {
                DispatchTarget::Cpu
            }
        }
        TaskTarget::Auto => {
            // Heuristic: prefer GPU for large workloads
            if gpu_available && task.data_size > 1024 * 1024 {
                DispatchTarget::Gpu
            } else if asm_enabled {
                DispatchTarget::CpuAsm
            } else {
                DispatchTarget::Cpu
            }
        }
    }
}
