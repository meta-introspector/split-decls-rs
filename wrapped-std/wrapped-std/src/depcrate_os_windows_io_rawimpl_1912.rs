// Generated macro for impl_1912 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1912 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1912"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl AsRawHandle for fs :: File { # [inline] fn as_raw_handle (& self) -> RawHandle { self . as_inner () . as_raw_handle () as RawHandle } }
};
}
