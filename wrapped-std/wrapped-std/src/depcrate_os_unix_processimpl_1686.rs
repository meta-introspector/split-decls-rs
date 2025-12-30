// Generated macro for impl_1686 (impl)
macro_rules! Depcrate_os_unix_processimpl_1686 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1686"}
// Dependencies: {}
# [doc = " Creates a `ChildStderr` from the provided `OwnedFd`."] # [doc = ""] # [doc = " The provided file descriptor must point to a pipe"] # [doc = " with the `CLOEXEC` flag set."] # [stable (feature = "child_stream_from_fd" , since = "1.74.0")] impl From < OwnedFd > for process :: ChildStderr { # [inline] fn from (fd : OwnedFd) -> process :: ChildStderr { let fd = sys :: fd :: FileDesc :: from_inner (fd) ; let pipe = sys :: pipe :: AnonPipe :: from_inner (fd) ; process :: ChildStderr :: from_inner (pipe) } }
};
}
