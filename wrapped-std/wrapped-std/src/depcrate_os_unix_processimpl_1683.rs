// Generated macro for impl_1683 (impl)
macro_rules! Depcrate_os_unix_processimpl_1683 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1683"}
// Dependencies: {}
# [doc = " Creates a `ChildStdout` from the provided `OwnedFd`."] # [doc = ""] # [doc = " The provided file descriptor must point to a pipe"] # [doc = " with the `CLOEXEC` flag set."] # [stable (feature = "child_stream_from_fd" , since = "1.74.0")] impl From < OwnedFd > for process :: ChildStdout { # [inline] fn from (fd : OwnedFd) -> process :: ChildStdout { let fd = sys :: fd :: FileDesc :: from_inner (fd) ; let pipe = sys :: pipe :: AnonPipe :: from_inner (fd) ; process :: ChildStdout :: from_inner (pipe) } }
};
}
