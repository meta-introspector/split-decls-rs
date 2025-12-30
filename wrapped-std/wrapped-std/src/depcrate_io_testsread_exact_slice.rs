// Generated macro for read_exact_slice (function)
macro_rules! Depcrate_io_testsread_exact_slice {
() => {
// Module: crate::io::tests
// Provides: {"read_exact_slice"}
// Dependencies: {}
# [test] fn read_exact_slice () { let mut buf = [0 ; 4] ; let mut c = & b"" [..] ; assert_eq ! (c . read_exact (& mut buf) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; let mut c = & b"123" [..] ; assert_eq ! (c . read_exact (& mut buf) . unwrap_err () . kind () , io :: ErrorKind :: UnexpectedEof) ; assert_eq ! (& buf , & [0 ; 4]) ; let mut c = & b"1234" [..] ; c . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , b"1234") ; let mut c = & b"56789" [..] ; c . read_exact (& mut buf) . unwrap () ; assert_eq ! (& buf , b"5678") ; assert_eq ! (c , b"9") ; }
};
}
