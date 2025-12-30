// Generated macro for test_buffered_reader_read_to_end_consumes_buffer (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_read_to_end_consumes_buffer {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_read_to_end_consumes_buffer"}
// Dependencies: {}
# [test] fn test_buffered_reader_read_to_end_consumes_buffer () { let data : & [u8] = & [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] ; let mut reader = BufReader :: with_capacity (3 , data) ; let mut buf = Vec :: new () ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1 , 2] [..])) ; assert_eq ! (reader . read_to_end (& mut buf) . ok () , Some (8)) ; assert_eq ! (& buf , & [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7]) ; assert ! (reader . buffer () . is_empty ()) ; }
};
}
