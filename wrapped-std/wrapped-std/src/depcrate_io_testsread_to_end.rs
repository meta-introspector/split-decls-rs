// Generated macro for read_to_end (function)
macro_rules! Depcrate_io_testsread_to_end {
() => {
// Module: crate::io::tests
// Provides: {"read_to_end"}
// Dependencies: {}
# [test] fn read_to_end () { let mut c = Cursor :: new (& b"" [..]) ; let mut v = Vec :: new () ; assert_eq ! (c . read_to_end (& mut v) . unwrap () , 0) ; assert_eq ! (v , []) ; let mut c = Cursor :: new (& b"1" [..]) ; let mut v = Vec :: new () ; assert_eq ! (c . read_to_end (& mut v) . unwrap () , 1) ; assert_eq ! (v , b"1") ; let cap = if cfg ! (miri) { 1024 } else { 1024 * 1024 } ; let data = (0 .. cap) . map (| i | (i / 3) as u8) . collect :: < Vec < _ > > () ; let mut v = Vec :: new () ; let (a , b) = data . split_at (data . len () / 2) ; assert_eq ! (Cursor :: new (a) . read_to_end (& mut v) . unwrap () , a . len ()) ; assert_eq ! (Cursor :: new (b) . read_to_end (& mut v) . unwrap () , b . len ()) ; assert_eq ! (v , data) ; }
};
}
