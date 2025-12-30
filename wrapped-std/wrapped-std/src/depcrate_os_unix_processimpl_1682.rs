// Generated macro for impl_1682 (impl)
macro_rules! Depcrate_os_unix_processimpl_1682 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1682"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStdout > for OwnedFd { # [doc = " Takes ownership of a [`ChildStdout`](crate::process::ChildStdout)'s file descriptor."] # [inline] fn from (child_stdout : crate :: process :: ChildStdout) -> OwnedFd { child_stdout . into_inner () . into_inner () . into_inner () } }
};
}
