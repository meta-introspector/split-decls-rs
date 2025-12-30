// Generated macro for test_short_reads (function)
macro_rules! Depcrate_io_buffered_teststest_short_reads {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_short_reads"}
// Dependencies: {}
# [test] fn test_short_reads () { let inner = ShortReader { lengths : vec ! [0 , 1 , 2 , 0 , 1 , 0] } ; let mut reader = BufReader :: new (inner) ; let mut buf = [0 , 0] ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 2) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 1) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; }
};
}
