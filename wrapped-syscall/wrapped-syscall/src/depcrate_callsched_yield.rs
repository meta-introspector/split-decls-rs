// Generated macro for sched_yield (function)
macro_rules! Depcrate_callsched_yield {
() => {
// Module: crate::call
// Provides: {"sched_yield"}
// Dependencies: {}
# [doc = " Yield the process's time slice to the kernel"] # [doc = ""] # [doc = " This function will return Ok(0) on success"] pub fn sched_yield () -> Result < usize > { unsafe { syscall0 (SYS_YIELD) } }
};
}
