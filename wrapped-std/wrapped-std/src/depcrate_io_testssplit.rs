// Generated macro for split (function)
macro_rules! Depcrate_io_testssplit {
() => {
// Module: crate::io::tests
// Provides: {"split"}
// Dependencies: {}
# [test] fn split () { let buf = Cursor :: new (& b"12" [..]) ; let mut s = buf . split (b'3') ; assert_eq ! (s . next () . unwrap () . unwrap () , vec ! [b'1' , b'2']) ; assert ! (s . next () . is_none ()) ; let buf = Cursor :: new (& b"1233" [..]) ; let mut s = buf . split (b'3') ; assert_eq ! (s . next () . unwrap () . unwrap () , vec ! [b'1' , b'2']) ; assert_eq ! (s . next () . unwrap () . unwrap () , vec ! []) ; assert ! (s . next () . is_none ()) ; }
};
}
