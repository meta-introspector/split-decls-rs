// Generated macro for impl_1900 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1900 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1900"}
// Dependencies: {}
# [stable (feature = "anonymous_pipe" , since = "1.87.0")] impl From < OwnedHandle > for io :: PipeWriter { fn from (owned_handle : OwnedHandle) -> Self { Self :: from_inner (FromInner :: from_inner (owned_handle)) } }
};
}
