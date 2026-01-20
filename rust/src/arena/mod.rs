//! Memory Arena Management
//!
//! Rust-controlled memory arenas for safe, fast allocation.
//! CUDA and ASM never do free malloc - all memory flows through here.

mod allocator;

pub use allocator::*;

/// Memory arena for controlled allocations
pub struct Arena {
    /// Base pointer
    base: *mut u8,
    /// Current offset
    offset: usize,
    /// Total capacity
    capacity: usize,
}

impl Arena {
    /// Create a new arena with given capacity
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::from_size_align(capacity, 16).unwrap();
        let base = unsafe { std::alloc::alloc(layout) };
        Self {
            base,
            offset: 0,
            capacity,
        }
    }

    /// Allocate bytes from the arena
    pub fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        let aligned_size = (size + 15) & !15; // 16-byte alignment
        if self.offset + aligned_size > self.capacity {
            return None;
        }
        let ptr = unsafe { self.base.add(self.offset) };
        self.offset += aligned_size;
        Some(ptr)
    }

    /// Reset the arena (free all allocations)
    pub fn reset(&mut self) {
        self.offset = 0;
    }

    /// Get remaining capacity
    pub fn remaining(&self) -> usize {
        self.capacity - self.offset
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.capacity, 16).unwrap();
        unsafe { std::alloc::dealloc(self.base, layout) };
    }
}
