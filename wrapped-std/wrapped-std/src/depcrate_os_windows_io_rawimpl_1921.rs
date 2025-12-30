// Generated macro for impl_1921 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1921 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1921"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawHandle for fs :: File { # [inline] fn into_raw_handle (self) -> RawHandle { self . into_inner () . into_raw_handle () as * mut _ } }
};
}
