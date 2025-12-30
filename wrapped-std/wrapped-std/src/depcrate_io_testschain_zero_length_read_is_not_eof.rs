// Generated macro for chain_zero_length_read_is_not_eof (function)
macro_rules! Depcrate_io_testschain_zero_length_read_is_not_eof {
() => {
// Module: crate::io::tests
// Provides: {"chain_zero_length_read_is_not_eof"}
// Dependencies: {}
# [test] fn chain_zero_length_read_is_not_eof () { let a = b"A" ; let b = b"B" ; let mut s = String :: new () ; let mut chain = (& a [..]) . chain (& b [..]) ; chain . read (& mut []) . unwrap () ; chain . read_to_string (& mut s) . unwrap () ; assert_eq ! ("AB" , s) ; }
};
}
