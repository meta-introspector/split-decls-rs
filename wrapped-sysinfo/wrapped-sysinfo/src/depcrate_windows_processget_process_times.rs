// Generated macro for get_process_times (function)
macro_rules! Depcrate_windows_processget_process_times {
() => {
// Module: crate::windows::process
// Provides: {"get_process_times"}
// Dependencies: {}
# [inline] unsafe fn get_process_times (handle : HANDLE) -> u64 { unsafe { let mut fstart : FILETIME = zeroed () ; let mut x = zeroed () ; let _err = GetProcessTimes (handle , & mut fstart as * mut FILETIME , & mut x as * mut FILETIME , & mut x as * mut FILETIME , & mut x as * mut FILETIME ,) ; filetime_to_u64 (fstart) } }
};
}
