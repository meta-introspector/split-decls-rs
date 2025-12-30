// Generated macro for impl_2486 (impl)
macro_rules! Depcrate_os_solid_ioimpl_2486 {
() => {
// Module: crate::os::solid::io
// Provides: {"impl_2486"}
// Dependencies: {}
impl BorrowedFd < '_ > { # [doc = " Creates a new `OwnedFd` instance that shares the same underlying file"] # [doc = " description as the existing `BorrowedFd` instance."] pub fn try_clone_to_owned (& self) -> crate :: io :: Result < OwnedFd > { let fd = sys :: net :: cvt (unsafe { crate :: sys :: abi :: sockets :: dup (self . as_raw_fd ()) }) ? ; Ok (unsafe { OwnedFd :: from_raw_fd (fd) }) } }
};
}
