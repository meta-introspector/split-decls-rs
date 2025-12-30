// Generated macro for impl_1993 (impl)
macro_rules! Depcrate_os_windows_processimpl_1993 {
() => {
// Module: crate::os::windows::process
// Provides: {"impl_1993"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsHandle for process :: Child { # [inline] fn as_handle (& self) -> BorrowedHandle < '_ > { self . as_inner () . handle () . as_handle () } }
};
}
