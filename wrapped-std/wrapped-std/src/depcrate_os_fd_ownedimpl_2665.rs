// Generated macro for impl_2665 (impl)
macro_rules! Depcrate_os_fd_ownedimpl_2665 {
() => {
// Module: crate::os::fd::owned
// Provides: {"impl_2665"}
// Dependencies: {}
impl BorrowedFd < '_ > { # [doc = " Creates a new `OwnedFd` instance that shares the same underlying file"] # [doc = " description as the existing `BorrowedFd` instance."] # [cfg (not (any (target_arch = "wasm32" , target_os = "hermit" , target_os = "trusty")))] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone_to_owned (& self) -> crate :: io :: Result < OwnedFd > { # [cfg (not (any (target_os = "espidf" , target_os = "vita")))] let cmd = libc :: F_DUPFD_CLOEXEC ; # [cfg (any (target_os = "espidf" , target_os = "vita"))] let cmd = libc :: F_DUPFD ; let fd = cvt (unsafe { libc :: fcntl (self . as_raw_fd () , cmd , 3) }) ? ; Ok (unsafe { OwnedFd :: from_raw_fd (fd) }) } # [doc = " Creates a new `OwnedFd` instance that shares the same underlying file"] # [doc = " description as the existing `BorrowedFd` instance."] # [cfg (any (target_arch = "wasm32" , target_os = "hermit" , target_os = "trusty"))] # [stable (feature = "io_safety" , since = "1.63.0")] pub fn try_clone_to_owned (& self) -> crate :: io :: Result < OwnedFd > { Err (crate :: io :: Error :: UNSUPPORTED_PLATFORM) } }
};
}
