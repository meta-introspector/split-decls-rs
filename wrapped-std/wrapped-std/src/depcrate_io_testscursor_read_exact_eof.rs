// Generated macro for cursor_read_exact_eof (function)
macro_rules! Depcrate_io_testscursor_read_exact_eof {
() => {
// Module: crate::io::tests
// Provides: {"cursor_read_exact_eof"}
// Dependencies: {}
# [test] fn cursor_read_exact_eof () { let slice = Cursor :: new (b"123456") ; let mut r = slice . clone () ; assert ! (r . read_exact (& mut [0 ; 10]) . is_err ()) ; assert ! (Cursor :: split (& r) . 1 . is_empty ()) ; let mut r = slice ; let buf = & mut [0 ; 10] ; let mut buf = BorrowedBuf :: from (buf . as_mut_slice ()) ; assert ! (r . read_buf_exact (buf . unfilled ()) . is_err ()) ; assert ! (Cursor :: split (& r) . 1 . is_empty ()) ; assert_eq ! (buf . filled () , b"123456") ; }
};
}
