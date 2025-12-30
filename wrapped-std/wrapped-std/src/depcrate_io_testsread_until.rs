// Generated macro for read_until (function)
macro_rules! Depcrate_io_testsread_until {
() => {
// Module: crate::io::tests
// Provides: {"read_until"}
// Dependencies: {}
# [test] fn read_until () { let mut buf = Cursor :: new (& b"12" [..]) ; let mut v = Vec :: new () ; assert_eq ! (buf . read_until (b'3' , & mut v) . unwrap () , 2) ; assert_eq ! (v , b"12") ; let mut buf = Cursor :: new (& b"1233" [..]) ; let mut v = Vec :: new () ; assert_eq ! (buf . read_until (b'3' , & mut v) . unwrap () , 3) ; assert_eq ! (v , b"123") ; v . truncate (0) ; assert_eq ! (buf . read_until (b'3' , & mut v) . unwrap () , 1) ; assert_eq ! (v , b"3") ; v . truncate (0) ; assert_eq ! (buf . read_until (b'3' , & mut v) . unwrap () , 0) ; assert_eq ! (v , []) ; }
};
}
