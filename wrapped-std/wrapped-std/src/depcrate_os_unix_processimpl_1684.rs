// Generated macro for impl_1684 (impl)
macro_rules! Depcrate_os_unix_processimpl_1684 {
() => {
// Module: crate::os::unix::process
// Provides: {"impl_1684"}
// Dependencies: {}
# [stable (feature = "io_safety" , since = "1.63.0")] impl AsFd for crate :: process :: ChildStderr { # [inline] fn as_fd (& self) -> BorrowedFd < '_ > { self . as_inner () . as_fd () } }
};
}
