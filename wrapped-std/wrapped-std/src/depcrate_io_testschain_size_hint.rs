// Generated macro for chain_size_hint (function)
macro_rules! Depcrate_io_testschain_size_hint {
() => {
// Module: crate::io::tests
// Provides: {"chain_size_hint"}
// Dependencies: {}
# [test] fn chain_size_hint () { let testdata = b"ABCDEFGHIJKL" ; let mut buf_reader_1 = BufReader :: new (& testdata [.. 6]) ; let mut buf_reader_2 = BufReader :: new (& testdata [6 ..]) ; buf_reader_1 . fill_buf () . unwrap () ; buf_reader_2 . fill_buf () . unwrap () ; let chain = buf_reader_1 . chain (buf_reader_2) ; let size_hint = chain . bytes () . size_hint () ; assert_eq ! (size_hint , (testdata . len () , Some (testdata . len ()))) ; }
};
}
