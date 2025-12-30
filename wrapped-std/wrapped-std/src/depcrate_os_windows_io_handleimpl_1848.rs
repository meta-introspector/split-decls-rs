// Generated macro for impl_1848 (impl)
macro_rules! Depcrate_os_windows_io_handleimpl_1848 {
() => {
// Module: crate::os::windows::io::handle
// Provides: {"impl_1848"}
// Dependencies: {}
impl OwnedHandle { # [doc = " Creates a new `OwnedHandle` instance that shares the same underlying"] # [doc = " object as the existing `OwnedHandle` instance."] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone (& self) -> crate :: io :: Result < Self > { self . as_handle () . try_clone_to_owned () } }
};
}
