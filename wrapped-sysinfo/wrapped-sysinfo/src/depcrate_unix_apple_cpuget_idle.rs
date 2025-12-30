// Generated macro for get_idle (function)
macro_rules! Depcrate_unix_apple_cpuget_idle {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"get_idle"}
// Dependencies: {}
# [inline] fn get_idle (cpu_info : * mut i32 , offset : isize) -> i32 { unsafe { * cpu_info . offset (offset + libc :: CPU_STATE_IDLE as isize) } }
};
}
