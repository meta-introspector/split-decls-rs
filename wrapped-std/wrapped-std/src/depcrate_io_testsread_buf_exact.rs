// Generated macro for read_buf_exact (function)
macro_rules! Depcrate_io_testsread_buf_exact {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_exact"}
// Dependencies: {}
# [test] fn read_buf_exact () { let buf : & mut [_] = & mut [0 ; 4] ; let mut buf : BorrowedBuf < '_ > = buf . into () ; let mut c = Cursor :: new (& b"" [..]) ; assert_eq ! (c . read_buf_exact (buf . unfilled ()) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; let mut c = Cursor :: new (& b"123456789" [..]) ; c . read_buf_exact (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , b"1234") ; buf . clear () ; c . read_buf_exact (buf . unfilled ()) . unwrap () ; assert_eq ! (buf . filled () , b"5678") ; buf . clear () ; assert_eq ! (c . read_buf_exact (buf . unfilled ()) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; }
};
}
