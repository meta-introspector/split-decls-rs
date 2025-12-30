// Generated macro for lines (function)
macro_rules! Depcrate_io_testslines {
() => {
// Module: crate::io::tests
// Provides: {"lines"}
// Dependencies: {}
# [test] fn lines () { let buf = Cursor :: new (& b"12\r" [..]) ; let mut s = buf . lines () ; assert_eq ! (s . next () . unwrap () . unwrap () , "12\r" . to_string ()) ; assert ! (s . next () . is_none ()) ; let buf = Cursor :: new (& b"12\r\n\n" [..]) ; let mut s = buf . lines () ; assert_eq ! (s . next () . unwrap () . unwrap () , "12" . to_string ()) ; assert_eq ! (s . next () . unwrap () . unwrap () , "" . to_string ()) ; assert ! (s . next () . is_none ()) ; }
};
}
