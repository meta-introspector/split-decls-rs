// Generated macro for get_start_time (function)
macro_rules! Depcrate_windows_processget_start_time {
() => {
// Module: crate::windows::process
// Provides: {"get_start_time"}
// Dependencies: {}
# [inline] pub (crate) fn get_start_time (handle : HANDLE) -> u64 { unsafe { let process_times = get_process_times (handle) ; compute_start (process_times) } }
};
}
