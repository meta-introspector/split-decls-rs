// Generated macro for impl_1938 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1938 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1938"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl FromRawHandle for io :: PipeWriter { unsafe fn from_raw_handle (raw_handle : RawHandle) -> Self { unsafe { Self :: from_inner (FromRawHandle :: from_raw_handle (raw_handle)) } } }
};
}
