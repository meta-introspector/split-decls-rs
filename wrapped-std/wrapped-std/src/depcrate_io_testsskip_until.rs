// Generated macro for skip_until (function)
macro_rules! Depcrate_io_testsskip_until {
() => {
// Module: crate::io::tests
// Provides: {"skip_until"}
// Dependencies: {}
# [test] fn skip_until () { let bytes : & [u8] = b"read\0ignore\0read\0ignore\0read\0ignore\0" ; let mut reader = BufReader :: new (bytes) ; loop { let mut out = Vec :: new () ; let read = reader . read_until (0 , & mut out) . unwrap () ; if read == 0 { break ; } else { assert_eq ! (out , b"read\0") ; assert_eq ! (read , b"read\0" . len ()) ; } let skipped = reader . skip_until (0) . unwrap () ; assert_eq ! (skipped , b"ignore\0" . len ()) ; } let skipped = reader . skip_until (0) . unwrap () ; assert_eq ! (skipped , 0) ; }
};
}
