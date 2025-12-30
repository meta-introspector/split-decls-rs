// Generated macro for impl_1998 (impl)
macro_rules! Depcrate_os_windows_processimpl_1998 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1998"}
// Dependencies: {}
# [stable (feature = "process_extensions" , since = "1.2.0")] impl AsRawHandle for process :: ChildStderr { # [inline] fn as_raw_handle (& self) -> RawHandle { self . as_inner () . handle () . as_raw_handle () as * mut _ } }
};
}
