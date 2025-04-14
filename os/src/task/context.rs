//! Implementation of [`TaskContext`]

use super::task::TaskTraceMap;

#[derive(Copy, Clone)]
#[repr(C)]
/// task context structure containing some registers
pub struct TaskContext {
    /// Ret position after task switching
    ra: usize,
    /// Stack pointer
    sp: usize,
    /// s0-11 register, callee saved
    s: [usize; 12],
}

impl TaskContext {
    /// Create a new empty task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    /// Create a new task context with a trap return addr and a kernel stack pointer
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }

    /// init trace map
    pub fn trace_init() -> TaskTraceMap {
        TaskTraceMap {
            syscall_list: [(64, 0), (93, 1), (124, 2), (169, 3), (410, 4)],
            trace_num_list: [0,0,0,0,0]
        }
    }
}
