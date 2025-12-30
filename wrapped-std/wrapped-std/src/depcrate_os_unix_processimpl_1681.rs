// Generated macro for impl_1681 (impl)
macro_rules! Depcrate_os_unix_processimpl_1681 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1681"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for crate :: process :: ChildStdout { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . as_inner () . as_fd () } }
};
}
