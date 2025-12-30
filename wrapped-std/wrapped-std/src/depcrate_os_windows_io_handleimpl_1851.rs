// Generated macro for impl_1851 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1851 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1851"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl Drop for HandleOrInvalid { # [inline] fn drop (& mut self) { if self . is_valid () { unsafe { let _ = sys :: c :: CloseHandle (self . 0) ; } } } }
};
}
