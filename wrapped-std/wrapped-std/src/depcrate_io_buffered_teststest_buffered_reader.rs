// Generated macro for test_buffered_reader (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader"}
// Dependencies: {}
# [test] fn test_buffered_reader () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (2 , inner) ; let mut buf = [0 , 0 , 0] ; let nread = reader . read (& mut buf) ; assert_eq ! (nread . unwrap () , 3) ; assert_eq ! (buf , [5 , 6 , 7]) ; assert_eq ! (reader . buffer () , []) ; let mut buf = [0 , 0] ; let nread = reader . read (& mut buf) ; assert_eq ! (nread . unwrap () , 2) ; assert_eq ! (buf , [0 , 1]) ; assert_eq ! (reader . buffer () , []) ; let mut buf = [0] ; let nread = reader . read (& mut buf) ; assert_eq ! (nread . unwrap () , 1) ; assert_eq ! (buf , [2]) ; assert_eq ! (reader . buffer () , [3]) ; let mut buf = [0 , 0 , 0] ; let nread = reader . read (& mut buf) ; assert_eq ! (nread . unwrap () , 1) ; assert_eq ! (buf , [3 , 0 , 0]) ; assert_eq ! (reader . buffer () , []) ; let nread = reader . read (& mut buf) ; assert_eq ! (nread . unwrap () , 1) ; assert_eq ! (buf , [4 , 0 , 0]) ; assert_eq ! (reader . buffer () , []) ; assert_eq ! (reader . read (& mut buf) . unwrap () , 0) ; }
};
}
