// Generated macro for chain_splitted_char (function)
macro_rules! Depcrate_io_testschain_splitted_char {
() => {
// Module: crate::io::tests
// Provides: {"chain_splitted_char"}
// Dependencies: {}
# [test] fn chain_splitted_char () { let chain = b"\xc3" . chain (b"\xa9" . as_slice ()) ; assert_eq ! (crate :: io :: read_to_string (chain) . unwrap () , "é") ; let mut chain = b"\xc3" . chain (b"\xa9\n" . as_slice ()) ; let mut buf = String :: new () ; assert_eq ! (chain . read_line (& mut buf) . unwrap () , 3) ; assert_eq ! (buf , "é\n") ; }
};
}
