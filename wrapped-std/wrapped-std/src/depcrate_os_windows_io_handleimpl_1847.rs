// Generated macro for impl_1847 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1847 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1847"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl Drop for HandleOrNull { # [inline] fn drop (& mut self) { if self . is_valid () { unsafe { let _ = sys :: c :: CloseHandle (self . 0) ; } } } }
};
}
