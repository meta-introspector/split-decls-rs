// Generated macro for test_lines (function)
macro_rules! Depcrate_io_buffered_teststest_lines {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_lines"}
// Dependencies: {}
# [test] fn test_lines () { let in_buf : & [u8] = b"a\nb\nc" ; let reader = BufReader :: with_capacity (2 , in_buf) ; let mut it = reader . lines () ; assert_eq ! (it . next () . unwrap () . unwrap () , "a" . to_string ()) ; assert_eq ! (it . next () . unwrap () . unwrap () , "b" . to_string ()) ; assert_eq ! (it . next () . unwrap () . unwrap () , "c" . to_string ()) ; assert ! (it . next () . is_none ()) ; }
};
}
