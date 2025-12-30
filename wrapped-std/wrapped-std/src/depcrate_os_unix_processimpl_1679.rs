// Generated macro for impl_1679 (impl)
macro_rules! Depcrate_os_unix_processimpl_1679 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1679"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl From < crate :: process :: ChildStdin > for OwnedFd { # [doc = " Takes ownership of a [`ChildStdin`](crate::process::ChildStdin)'s file descriptor."] # [inline] fn from (child_stdin : crate :: process :: ChildStdin) -> OwnedFd { child_stdin . into_inner () . into_inner () . into_inner () } }
};
}
