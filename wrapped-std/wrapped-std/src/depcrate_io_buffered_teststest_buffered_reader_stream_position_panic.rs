// Generated macro for test_buffered_reader_stream_position_panic (function)
macro_rules! Depcrate_io_buffered_teststest_buffered_reader_stream_position_panic {
() => {
// Module: crate::io::buffered::tests
// Provides: {"test_buffered_reader_stream_position_panic"}
// Dependencies: {}
# [test] # [cfg_attr (not (panic = "unwind") , ignore = "test requires unwinding support")] fn test_buffered_reader_stream_position_panic () { let inner : & [u8] = & [5 , 6 , 7 , 0 , 1 , 2 , 3 , 4] ; let mut reader = BufReader :: with_capacity (4 , io :: Cursor :: new (inner)) ; let mut buffer = [0 , 0] ; assert ! (reader . read_exact (& mut buffer) . is_ok ()) ; let inner = reader . get_mut () ; assert ! (inner . seek (SeekFrom :: Start (0)) . is_ok ()) ; let result = panic :: catch_unwind (panic :: AssertUnwindSafe (| | reader . stream_position () . ok ())) ; assert ! (result . is_err ()) ; }
};
}
