// Generated macro for test_read_line (function)
macro_rules! Depcrate_io_buffered_teststest_read_line {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_read_line"}
// Dependencies: {}
# [test] fn test_read_line () { let in_buf : & [u8] = b"a\nb\nc" ; let mut reader = BufReader :: with_capacity (2 , in_buf) ; let mut s = String :: new () ; reader . read_line (& mut s) . unwrap () ; assert_eq ! (s , "a\n") ; s . truncate (0) ; reader . read_line (& mut s) . unwrap () ; assert_eq ! (s , "b\n") ; s . truncate (0) ; reader . read_line (& mut s) . unwrap () ; assert_eq ! (s , "c") ; s . truncate (0) ; reader . read_line (& mut s) . unwrap () ; assert_eq ! (s , "") ; }
};
}
