// Generated macro for impl_1685 (impl)
macro_rules! Depcrate_os_unix_processimpl_1685 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1685"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStderr > for OwnedFd { # [doc = " Takes ownership of a [`ChildStderr`](crate::process::ChildStderr)'s file descriptor."] # [inline] fn from (child_stderr : crate :: process :: ChildStderr) -> OwnedFd { child_stderr . into_inner () . into_inner () . into_inner () } }
};
}
