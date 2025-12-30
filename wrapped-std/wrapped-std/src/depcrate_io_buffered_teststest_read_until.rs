// Generated macro for test_read_until (function)
macro_rules! Depcrate_io_buffered_teststest_read_until {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_read_until"}
// Dependencies: {}
# [test] fn test_read_until () { let inner : & [u8] = & [0 , 1 , 2 , 1 , 0] ; let mut reader = BufReader :: with_capacity (2 , inner) ; let mut v = Vec :: new () ; reader . read_until (0 , & mut v) . unwrap () ; assert_eq ! (v , [0]) ; v . truncate (0) ; reader . read_until (2 , & mut v) . unwrap () ; assert_eq ! (v , [1 , 2]) ; v . truncate (0) ; reader . read_until (1 , & mut v) . unwrap () ; assert_eq ! (v , [1]) ; v . truncate (0) ; reader . read_until (8 , & mut v) . unwrap () ; assert_eq ! (v , [0]) ; v . truncate (0) ; reader . read_until (9 , & mut v) . unwrap () ; assert_eq ! (v , []) ; }
};
}
