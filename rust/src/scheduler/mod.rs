//! Task Scheduler
//!
//! Decides what executes, when, and where (CPU/GPU).
//! Central dispatch for all workloads.

mod task;
mod dispatch;

pub use task::*;
pub use dispatch::*;

/// Scheduler configuration
pub struct SchedulerConfig {
    /// Maximum concurrent tasks
    pub max_tasks: usize,
    /// Prefer GPU when available
    pub prefer_gpu: bool,
    /// Enable ASM hot paths
    pub enable_asm: bool,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            max_tasks: 64,
            prefer_gpu: true,
            enable_asm: true,
        }
    }
}

/// Main scheduler instance
pub struct Scheduler {
    config: SchedulerConfig,
    // TODO: task queue, thread pool, etc.
}

impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        Self { config }
    }

    /// Submit a task for execution
    pub fn submit(&mut self, task: Task) -> TaskHandle {
        // TODO: actual scheduling logic
        TaskHandle { id: 0 }
    }

    /// Wait for a task to complete
    pub fn wait(&self, handle: TaskHandle) -> TaskResult {
        // TODO: actual wait logic
        TaskResult::Success
    }
}
