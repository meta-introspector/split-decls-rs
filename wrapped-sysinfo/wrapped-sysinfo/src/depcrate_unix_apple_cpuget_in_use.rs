// Generated macro for get_in_use (function)
macro_rules! Depcrate_unix_apple_cpuget_in_use {
() => {
// Module: crate::unix::apple::cpu
// Provides: {"get_in_use"}
// Dependencies: {}
# [inline] fn get_in_use (cpu_info : * mut i32 , offset : isize) -> i64 { unsafe { let user = * cpu_info . offset (offset + libc :: CPU_STATE_USER as isize) as i64 ; let system = * cpu_info . offset (offset + libc :: CPU_STATE_SYSTEM as isize) as i64 ; let nice = * cpu_info . offset (offset + libc :: CPU_STATE_NICE as isize) as i64 ; user . saturating_add (system) . saturating_add (nice) } }
};
}
