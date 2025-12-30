// Generated macro for test_buffered_reader_seek_relative (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_seek_relative {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_seek_relative"}
// Dependencies: {}
# [test] fn test_buffered_reader_seek_relative () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (2 , io :: Cursor :: new (inner)) ; assert ! (reader . seek_relative (3) . is_ok ()) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert ! (reader . seek_relative (0) . is_ok ()) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert ! (reader . seek_relative (1) . is_ok ()) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [1] [..])) ; assert ! (reader . seek_relative (- 1) . is_ok ()) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert ! (reader . seek_relative (2) . is_ok ()) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [2 , 3] [..])) ; }
};
}
