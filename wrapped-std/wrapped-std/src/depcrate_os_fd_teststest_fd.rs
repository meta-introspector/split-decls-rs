// Generated macro for test_fd (function)
macro_rules! Depcrate_os_fd_teststest_fd {
() => {
// Module: crate::os::fd::tests
// Provides: {"test_fd"}
// Dependencies: {}
# [cfg (any (unix , target_os = "wasi"))] # [test] fn test_fd () { # [cfg (unix)] use crate :: os :: unix :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; # [cfg (target_os = "wasi")] use crate :: os :: wasi :: io :: { AsFd , AsRawFd , BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; let stdin = crate :: io :: stdin () ; let fd : BorrowedFd < '_ > = stdin . as_fd () ; let raw_fd : RawFd = fd . as_raw_fd () ; let owned_fd : OwnedFd = unsafe { OwnedFd :: from_raw_fd (raw_fd) } ; let stdin_as_file = crate :: fs :: File :: from (owned_fd) ; assert_eq ! (stdin_as_file . as_fd () . as_raw_fd () , raw_fd) ; assert_eq ! (Into ::< OwnedFd >:: into (stdin_as_file) . into_raw_fd () , raw_fd) ; }
};
}
