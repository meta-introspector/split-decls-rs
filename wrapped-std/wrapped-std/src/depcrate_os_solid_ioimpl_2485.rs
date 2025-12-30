// Generated macro for impl_2485 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2485 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2485"}
// Dependencies: {}
impl OwnedFd { # [doc = " Creates a new `OwnedFd` instance that shares the same underlying file"] # [doc = " description as the existing `OwnedFd` instance."] pub fn try_clone (& self) -> crate :: io :: Result < Self > { self . as_fd () . try_clone_to_owned () } }
};
}
