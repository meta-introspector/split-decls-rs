// Generated macro for get_accumulated_cpu_time (function)
macro_rules! Depcrate_unix_freebsd_processget_accumulated_cpu_time {
() => {
// Module: crate::unix::freebsd::process
// Provides: {"get_accumulated_cpu_time"}
// Dependencies: {}
# [inline] fn get_accumulated_cpu_time (kproc : & libc :: kinfo_proc) -> u64 { kproc . ki_runtime / 1_000 }
};
}
