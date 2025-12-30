// Generated macro for test_writer_read_from_multiple_bufs (function)
macro_rules! Depcrate_io_teststest_writer_read_from_multiple_bufs {
() => {
// Module: crate::io::tests
// Provides: {"test_writer_read_from_multiple_bufs"}
// Dependencies: {}
# [test] fn test_writer_read_from_multiple_bufs () { let mut writer = test_writer (3 , 3) ; let bufs = & [IoSlice :: new (& [1]) , IoSlice :: new (& [2 , 2 , 2])] ; assert_eq ! (writer . write_vectored (bufs) . unwrap () , 3) ; let bufs = & [IoSlice :: new (& [3]) , IoSlice :: new (& [4]) , IoSlice :: new (& [5 , 5])] ; assert_eq ! (writer . write_vectored (bufs) . unwrap () , 3) ; assert_eq ! (writer . written , & [1 , 2 , 2 , 3 , 4 , 5]) ; }
};
}
