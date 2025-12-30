// Generated macro for test_raw_fd (function)
macro_rules! Depcrate_os_fd_teststest_raw_fd {
() => {
// Module: crate::os::fd::tests
// Provides: {"test_raw_fd"}
// Dependencies: {}
# [cfg (any (unix , target_os = "wasi"))] # [test] fn test_raw_fd () { # [cfg (unix)] use crate :: os :: unix :: io :: { AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ; # [cfg (target_os = "wasi")] use crate :: os :: wasi :: io :: { AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , RawFd } ; let raw_fd : RawFd = crate :: io :: stdin () . as_raw_fd () ; let stdin_as_file = unsafe { crate :: fs :: File :: from_raw_fd (raw_fd) } ; assert_eq ! (stdin_as_file . as_raw_fd () , raw_fd) ; assert_eq ! (unsafe { BorrowedFd :: borrow_raw (raw_fd) . as_raw_fd () } , raw_fd) ; assert_eq ! (stdin_as_file . into_raw_fd () , 0) ; }
};
}
