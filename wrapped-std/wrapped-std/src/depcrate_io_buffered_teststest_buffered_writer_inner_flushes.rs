// Generated macro for test_buffered_writer_inner_flushes (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_writer_inner_flushes {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_writer_inner_flushes"}
// Dependencies: {}
# [test] fn test_buffered_writer_inner_flushes () { let mut w = BufWriter :: with_capacity (3 , Vec :: new ()) ; w . write (& [0 , 1]) . unwrap () ; assert_eq ! (* w . get_ref () , []) ; let w = w . into_inner () . unwrap () ; assert_eq ! (w , [0 , 1]) ; }
};
}
