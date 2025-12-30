// Generated macro for impl_1894 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1894 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1894"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl < T > From < crate :: thread :: JoinHandle < T > > for OwnedHandle { # [inline] fn from (join_handle : crate :: thread :: JoinHandle < T >) -> OwnedHandle { join_handle . into_inner () . into_handle () . into_inner () } }
};
}
