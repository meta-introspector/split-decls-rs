// Generated macro for impl_1896 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1896 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1896"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl From < io :: PipeReader > for OwnedHandle { fn from (pipe : io :: PipeReader) -> Self { pipe . into_inner () . into_inner () } }
};
}
