// Generated macro for test_buffered_reader_stream_position (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_stream_position {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_stream_position"}
// Dependencies: {}
# [test] fn test_buffered_reader_stream_position () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (2 , io :: Cursor :: new (inner)) ; assert_eq ! (reader . stream_position () . ok () , Some (0)) ; assert_eq ! (reader . seek (SeekFrom :: Start (3)) . ok () , Some (3)) ; assert_eq ! (reader . stream_position () . ok () , Some (3)) ; assert_eq ! (reader . fill_buf () . ok () , Some (& [0 , 1] [..])) ; assert ! (reader . seek_relative (0) . is_ok ()) ; assert_eq ! (reader . stream_position () . ok () , Some (3)) ; assert_eq ! (reader . buffer () , & [0 , 1] [..]) ; assert ! (reader . seek_relative (1) . is_ok ()) ; assert_eq ! (reader . stream_position () . ok () , Some (4)) ; assert_eq ! (reader . buffer () , & [1] [..]) ; assert ! (reader . seek_relative (- 1) . is_ok ()) ; assert_eq ! (reader . stream_position () . ok () , Some (3)) ; assert_eq ! (reader . buffer () , & [0 , 1] [..]) ; assert ! (reader . seek_relative (2) . is_ok ()) ; assert_eq ! (reader . stream_position () . ok () , Some (5)) ; assert_eq ! (reader . buffer () , & [] [..]) ; }
};
}
