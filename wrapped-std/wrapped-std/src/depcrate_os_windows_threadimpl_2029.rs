// Generated macro for impl_2029 (impl)
macro_rules! Depcrate_os_windows_threadimpl_2029 {
() => {
// Module: crate::os::windows::thread
// Provides: {"impl_2029"}
// Dependencies: {}
# [stable (feature = "thread_extensions" , since = "1.9.0")] impl < T > IntoRawHandle for thread :: JoinHandle < T > { # [inline] fn into_raw_handle (self) -> RawHandle { self . into_inner () . into_handle () . into_raw_handle () as * mut _ } }
};
}
