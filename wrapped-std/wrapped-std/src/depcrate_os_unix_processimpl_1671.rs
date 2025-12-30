// Generated macro for impl_1671 (impl)
macro_rules! Depcrate_os_unix_processimpl_1671 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1671"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < OwnedFd > for process :: Stdio { # [doc = " Takes ownership of a file descriptor and returns a [`Stdio`](process::Stdio)"] # [doc = " that can attach a stream to it."] # [inline] fn from (fd : OwnedFd) -> process :: Stdio { let fd = sys :: fd :: FileDesc :: from_inner (fd) ; let io = sys :: process :: Stdio :: Fd (fd) ; process :: Stdio :: from_inner (io) } }
};
}
