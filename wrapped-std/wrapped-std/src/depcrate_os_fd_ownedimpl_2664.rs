// Generated macro for impl_2664 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2664 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2664"}
// Dependencies: {}
impl OwnedFd { # [doc = " Creates a new `OwnedFd` instance that shares the same underlying file"] # [doc = " description as the existing `OwnedFd` instance."] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone (& self) -> crate :: io :: Result < Self > { self . as_fd () . try_clone_to_owned () } }
};
}
