// Generated macro for impl_2028 (impl)
macro_rules! Depcrate_os_windows_threadimpl_2028 {
() => {
// Module: crate::os::windows::thread
// Provides: {"impl_2028"}
// Dependencies: {}
# [stable (feature = "thread_extensions" , since = "1.9.0")] impl < T > AsRawHandle for thread :: JoinHandle < T > { # [inline] fn as_raw_handle (& self) -> RawHandle { self . as_inner () . handle () . as_raw_handle () as * mut _ } }
};
}
