// Generated macro for borrowed_cursor_advance_overflow (function)
macro_rules! Depcrate_io_testsborrowed_cursor_advance_overflow {
() => {
// Module: crate::io::tests
// Provides: {"borrowed_cursor_advance_overflow"}
// Dependencies: {}
# [test] # [should_panic] fn borrowed_cursor_advance_overflow () { let mut buf = [0 ; 512] ; let mut buf = BorrowedBuf :: from (& mut buf [..]) ; buf . unfilled () . advance (1) ; buf . unfilled () . advance (usize :: MAX) ; }
};
}
