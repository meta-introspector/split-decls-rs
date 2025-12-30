// Generated macro for impl_1860 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1860 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1860"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl IntoRawHandle for OwnedHandle { # [inline] fn into_raw_handle (self) -> RawHandle { ManuallyDrop :: new (self) . handle } }
};
}
