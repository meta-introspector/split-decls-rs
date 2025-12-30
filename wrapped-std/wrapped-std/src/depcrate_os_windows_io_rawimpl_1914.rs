// Generated macro for impl_1914 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1914 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1914"}
// Dependencies: {}
# [stable (feature = "asraw_stdio" , since = "1.21.0")] impl AsRawHandle for io :: Stdout { fn as_raw_handle (& self) -> RawHandle { stdio_handle (unsafe { sys :: c :: GetStdHandle (sys :: c :: STD_OUTPUT_HANDLE) as RawHandle }) } }
};
}
