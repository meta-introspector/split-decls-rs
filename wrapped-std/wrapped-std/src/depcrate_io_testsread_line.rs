// Generated macro for read_line (function)
macro_rules! Depcrate_io_testsread_line {
() => {
// Module: crate::io::tests
// Provides: {"read_line"}
// Dependencies: {}
# [test] fn read_line () { let mut buf = Cursor :: new (& b"12" [..]) ; let mut v = String :: new () ; assert_eq ! (buf . read_line (& mut v) . unwrap () , 2) ; assert_eq ! (v , "12") ; let mut buf = Cursor :: new (& b"12\n\n" [..]) ; let mut v = String :: new () ; assert_eq ! (buf . read_line (& mut v) . unwrap () , 3) ; assert_eq ! (v , "12\n") ; v . truncate (0) ; assert_eq ! (buf . read_line (& mut v) . unwrap () , 1) ; assert_eq ! (v , "\n") ; v . truncate (0) ; assert_eq ! (buf . read_line (& mut v) . unwrap () , 0) ; assert_eq ! (v , "") ; }
};
}
