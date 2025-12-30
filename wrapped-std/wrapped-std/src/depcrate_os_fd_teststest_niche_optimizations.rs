// Generated macro for test_niche_optimizations (function)
macro_rules! Depcrate_os_fd_teststest_niche_optimizations {
() => {
// Module: crate::os::fd::tests
// Provides: {"test_niche_optimizations"}
// Dependencies: {}
# [cfg (any (unix , target_os = "wasi"))] # [test] fn test_niche_optimizations () { # [cfg (unix)] use crate :: os :: unix :: io :: { BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; # [cfg (target_os = "wasi")] use crate :: os :: wasi :: io :: { BorrowedFd , FromRawFd , IntoRawFd , OwnedFd , RawFd } ; assert_eq ! (size_of ::< Option < OwnedFd >> () , size_of ::< RawFd > ()) ; assert_eq ! (size_of ::< Option < BorrowedFd <'static >>> () , size_of ::< RawFd > ()) ; unsafe { assert_eq ! (OwnedFd :: from_raw_fd (RawFd :: MIN) . into_raw_fd () , RawFd :: MIN) ; assert_eq ! (OwnedFd :: from_raw_fd (RawFd :: MAX) . into_raw_fd () , RawFd :: MAX) ; assert_eq ! (Some (OwnedFd :: from_raw_fd (RawFd :: MIN)) . unwrap () . into_raw_fd () , RawFd :: MIN) ; assert_eq ! (Some (OwnedFd :: from_raw_fd (RawFd :: MAX)) . unwrap () . into_raw_fd () , RawFd :: MAX) ; } }
};
}
