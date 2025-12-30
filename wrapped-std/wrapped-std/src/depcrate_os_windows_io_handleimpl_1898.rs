// Generated macro for impl_1898 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1898 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1898"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl From < io :: PipeWriter > for OwnedHandle { fn from (pipe : io :: PipeWriter) -> Self { pipe . into_inner () . into_inner () } }
};
}
