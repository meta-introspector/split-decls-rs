// Generated macro for impl_2000 (impl)
macro_rules! Depcrate_os_windows_processimpl_2000 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_2000"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawHandle for process :: ChildStdout { fn into_raw_handle (self) -> RawHandle { self . into_inner () . into_handle () . into_raw_handle () as * mut _ } }
};
}
