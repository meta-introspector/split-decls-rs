// Generated macro for test_buffered_reader_read_to_string_consumes_buffer (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_read_to_string_consumes_buffer {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_read_to_string_consumes_buffer"}
// Dependencies: {}
# [test] fn test_buffered_reader_read_to_string_consumes_buffer () { let data : & [u8] = "deadbeef" . as_bytes () ; let mut reader = BufReader :: with_capacity (3 , data) ; let mut buf = String :: new () ; assert_eq ! (reader . fill_buf () . ok () , Some ("dea" . as_bytes ())) ; assert_eq ! (reader . read_to_string (& mut buf) . ok () , Some (8)) ; assert_eq ! (& buf , "deadbeef") ; assert ! (reader . buffer () . is_empty ()) ; }
};
}
