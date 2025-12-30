// Generated macro for read_to_string (function)
macro_rules! Depcrate_io_testsread_to_string {
() => {
// Module: crate::io::tests
// Provides: {"read_to_string"}
// Dependencies: {}
# [test] fn read_to_string () { let mut c = Cursor :: new (& b"" [..]) ; let mut v = String :: new () ; assert_eq ! (c . read_to_string (& mut v) . unwrap () , 0) ; assert_eq ! (v , "") ; let mut c = Cursor :: new (& b"1" [..]) ; let mut v = String :: new () ; assert_eq ! (c . read_to_string (& mut v) . unwrap () , 1) ; assert_eq ! (v , "1") ; let mut c = Cursor :: new (& b"\xff" [..]) ; let mut v = String :: new () ; assert ! (c . read_to_string (& mut v) . is_err ()) ; }
};
}
