// Generated macro for test_buffered_reader_invalidated_after_read (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_invalidated_after_read {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_invalidated_after_read"}
// Dependencies: {}
# [test] fn test_buffered_reader_invalidated_after_read () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (3 , io :: Cursor :: new (inner)) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [5 , 6 , 7] [..])) ; reader . consume (3) ; let mut buffer = [0 , 0 , 0 , 0 , 0] ; assert_eq ! (reader . read (& mut buffer) . ok () , Some (5)) ; assert_eq ! (buffer , [0 , 1 , 2 , 3 , 4]) ; assert ! (reader . seek_relative (- 2) . is_ok ()) ; let mut buffer = [0 , 0] ; assert_eq ! (reader . read (& mut buffer) . ok () , Some (2)) ; assert_eq ! (buffer , [3 , 4]) ; }
};
}
