// Generated macro for get_start_and_run_time (function)
macro_rules! Depcrate_windows_processget_start_and_run_time {
() => {
// Module: crate::windows::process
// Provides: {"get_start_and_run_time"}
// Dependencies: {}
fn get_start_and_run_time (handle : HANDLE , now : u64) -> (u64 , u64) { unsafe { let process_times = get_process_times (handle) ; let start = compute_start (process_times) ; let run_time = check_sub (now , start) ; (start , run_time) } }
};
}
