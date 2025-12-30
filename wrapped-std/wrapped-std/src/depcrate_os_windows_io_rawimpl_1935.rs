// Generated macro for impl_1935 (impl)
macro_rules! Depcrate_os_windows_io_rawimpl_1935 {
() => {
// Module: crate::os::windows::io::raw
// Provides: {"impl_1935"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl FromRawHandle for io :: PipeReader { unsafe fn from_raw_handle (raw_handle : RawHandle) -> Self { unsafe { Self :: from_inner (FromRawHandle :: from_raw_handle (raw_handle)) } } }
};
}
