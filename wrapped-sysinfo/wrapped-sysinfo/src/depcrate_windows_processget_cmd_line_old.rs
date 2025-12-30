// Generated macro for get_cmd_line_old (function)
macro_rules! Depcrate_windows_processget_cmd_line_old {
() => {
// Module: crate::windows::process
// Provides: {"get_cmd_line_old"}
// Dependencies: {}
fn get_cmd_line_old < T : RtlUserProcessParameters > (params : & T , handle : HANDLE) -> Vec < OsString > { match params . get_cmdline (handle) { Ok (buffer) => unsafe { get_cmdline_from_buffer (PCWSTR :: from_raw (buffer . as_ptr ())) } , Err (_e) => { sysinfo_debug ! ("get_cmd_line_old failed to get data: {}" , _e) ; Vec :: new () } } }
};
}
