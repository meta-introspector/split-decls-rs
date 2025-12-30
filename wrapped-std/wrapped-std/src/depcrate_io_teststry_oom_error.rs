// Generated macro for try_oom_error (function)
macro_rules! Depcrate_io_teststry_oom_error {
() => {
// Module: crate::io::tests
// Provides: {"try_oom_error"}
// Dependencies: {}
# [test] fn try_oom_error () { use alloc :: alloc :: Layout ; use alloc :: collections :: { TryReserveError , TryReserveErrorKind } ; let layout = Layout :: new :: < u8 > () ; let kind = TryReserveErrorKind :: AllocError { layout , non_exhaustive : () } ; let reserve_err = TryReserveError :: from (kind) ; let io_err = io :: Error :: from (reserve_err) ; assert_eq ! (io :: ErrorKind :: OutOfMemory , io_err . kind ()) ; }
};
}
