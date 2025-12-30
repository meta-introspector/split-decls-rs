// Generated macro for impl_1999 (impl)
macro_rules! Depcrate_os_windows_processimpl_1999 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1999"}
// Dependencies: {}
# [stable (feature = "into_raw_os" , since = "1.4.0")] impl IntoRawHandle for process :: ChildStdin { fn into_raw_handle (self) -> RawHandle { self . into_inner () . into_handle () . into_raw_handle () as * mut _ } }
};
}
