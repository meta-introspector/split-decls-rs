// Generated macro for test_buffered_reader_seek (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_seek {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_seek"}
// Dependencies: {}
# [test] fn test_buffered_reader_seek () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (2 , io :: Cursor :: new (inner)) ; assert_eq ! (reader . seek (SeekFrom :: Start (3)) . ok () , Some (3)) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert_eq ! (reader . stream_position () . ok () , Some (3)) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert_eq ! (reader . seek (SeekFrom :: Current (1)) . ok () , Some (4)) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [1 , 2] [..])) ; reader . consume (1) ; assert_eq ! (reader . seek (SeekFrom :: Current (- 2)) . ok () , Some (3)) ; }
};
}
