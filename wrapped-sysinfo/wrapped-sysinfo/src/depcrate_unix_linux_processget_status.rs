// Generated macro for get_status (function)
macro_rules! Depcrate_unix_linux_processget_status {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_status"}
// Dependencies: {}
# [inline (always)] fn get_status (p : & mut ProcessInner , part : & str) { p . status = part . chars () . next () . map (ProcessStatus :: from) . unwrap_or_else (| | ProcessStatus :: Unknown (0)) ; }
};
}
