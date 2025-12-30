// Generated macro for get_cmd_line_new (function)
macro_rules! Depcrate_windows_processget_cmd_line_new {
() => {
// Module: crate::windows::process
// Provides: {"get_cmd_line_new"}
// Dependencies: {}
# [allow (clippy :: cast_ptr_alignment)] fn get_cmd_line_new (handle : HANDLE) -> Vec < OsString > { unsafe { if let Some (buffer) = ph_query_process_variable_size (handle , ProcessCommandLineInformation) { let buffer = (* (buffer . as_ptr () as * const UNICODE_STRING)) . Buffer ; get_cmdline_from_buffer (PCWSTR :: from_raw (buffer . as_ptr ())) } else { vec ! [] } } }
};
}
