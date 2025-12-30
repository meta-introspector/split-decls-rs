// Generated macro for compute_start_time_without_boot_time (function)
macro_rules! Depcrate_unix_linux_processcompute_start_time_without_boot_time {
() => {
// Module: crate::unix::linux::process
// Provides: {"compute_start_time_without_boot_time"}
// Dependencies: {}
# [inline (always)] fn compute_start_time_without_boot_time (parts : & Parts < '_ > , info : & SystemInfo) -> (u64 , u64) { let raw = start_time_raw (parts) ; (raw , raw / info . clock_cycle) }
};
}
