// Generated macro for test_buffered_writer_seek (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_writer_seek {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_writer_seek"}
// Dependencies: {}
# [test] fn test_buffered_writer_seek () { let mut w = BufWriter :: with_capacity (3 , io :: Cursor :: new (Vec :: new ())) ; w . write_all (& [0 , 1 , 2 , 3 , 4 , 5]) . unwrap () ; w . write_all (& [6 , 7]) . unwrap () ; assert_eq ! (w . stream_position () . ok () , Some (8)) ; assert_eq ! (& w . get_ref () . get_ref () [..] , & [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7] [..]) ; assert_eq ! (w . seek (SeekFrom :: Start (2)) . ok () , Some (2)) ; w . write_all (& [8 , 9]) . unwrap () ; assert_eq ! (& w . into_inner () . unwrap () . into_inner () [..] , & [0 , 1 , 8 , 9 , 4 , 5 , 6 , 7]) ; }
};
}
