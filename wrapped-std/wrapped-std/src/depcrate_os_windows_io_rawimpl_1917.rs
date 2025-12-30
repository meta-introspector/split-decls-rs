// Generated macro for impl_1917 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1917 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1917"}
// Dependencies: {}
# [stable (feature = "asraw_stdio_locks" , since = "1.35.0")] impl < 'a > AsRawHandle for io :: StdoutLock < 'a > { fn as_raw_handle (& self) -> RawHandle { stdio_handle (unsafe { sys :: c :: GetStdHandle (sys :: c :: STD_OUTPUT_HANDLE) as RawHandle }) } }
};
}
