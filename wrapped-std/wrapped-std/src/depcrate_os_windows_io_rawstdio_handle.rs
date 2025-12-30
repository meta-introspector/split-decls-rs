// Generated macro for stdio_handle (function)
macro_rules! Depcrate_os_windows_io_rawstdio_handle {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"stdio_handle"}
// Dependencies: {}
fn stdio_handle (raw : RawHandle) -> RawHandle { if raw == sys :: c :: INVALID_HANDLE_VALUE { ptr :: null_mut () } else { raw } }
};
}
