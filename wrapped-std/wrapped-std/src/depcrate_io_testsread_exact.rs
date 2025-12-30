// Generated macro for read_exact (function)
macro_rules! Depcrate_io_testsread_exact {
() => {
// Module: crate::io::tests
// Provides: {"read_exact"}
// Dependencies: {}
# [test] fn read_exact () { let mut buf = [0 ; 4] ; let mut c = Cursor :: new (& b"" [..]) ; assert_eq ! (c . read_exact (& mut buf) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; let mut c = Cursor :: new (& b"123" [..]) . chain (Cursor :: new (& b"456789" [..])) ; c . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , b"1234") ; c . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , b"5678") ; assert_eq ! (c . read_exact (& mut buf) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; }
};
}
