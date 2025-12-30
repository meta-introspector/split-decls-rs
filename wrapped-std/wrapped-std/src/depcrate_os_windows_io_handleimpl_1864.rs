// Generated macro for impl_1864 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1864 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1864"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl Drop for OwnedHandle { # [inline] fn drop (& mut self) { unsafe { let _ = sys :: c :: CloseHandle (self . handle) ; } } }
};
}
