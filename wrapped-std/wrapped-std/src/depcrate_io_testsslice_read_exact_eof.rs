// Generated macro for slice_read_exact_eof (function)
macro_rules! Depcrate_io_testsslice_read_exact_eof {
() => {
// Module: crate::io::tests
// Provides: {"slice_read_exact_eof"}
// Dependencies: {}
# [test] fn slice_read_exact_eof () { let slice = & b"123456" [..] ; let mut r = slice ; assert ! (r . read_exact (& mut [0 ; 10]) . is_err ()) ; assert ! (r . is_empty ()) ; let mut r = slice ; let buf = & mut [0 ; 10] ; let mut buf = BorrowedBuf :: from (buf . as_mut_slice ()) ; assert ! (r . read_buf_exact (buf . unfilled ()) . is_err ()) ; assert ! (r . is_empty ()) ; assert_eq ! (buf . filled () , b"123456") ; }
};
}
