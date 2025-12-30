// Generated macro for bufreader_size_hint (function)
macro_rules! Depcrate_io_testsbufreader_size_hint {
() => {
// Module: crate::io::tests
// Provides: {"bufreader_size_hint"}
// Dependencies: {}
# [test] fn bufreader_size_hint () { let testdata = b"ABCDEFGHIJKL" ; let mut buf_reader = BufReader :: new (& testdata [..]) ; assert_eq ! (buf_reader . buffer () . len () , 0) ; let buffer_length = testdata . len () ; buf_reader . fill_buf () . unwrap () ; let mut buffered_bytes = buf_reader . bytes () ; let (lower_bound , _upper_bound) = buffered_bytes . size_hint () ; assert_eq ! (lower_bound , buffer_length) ; buffered_bytes . next () . unwrap () . unwrap () ; let (lower_bound , _upper_bound) = buffered_bytes . size_hint () ; assert_eq ! (lower_bound , buffer_length - 1) ; }
};
}
