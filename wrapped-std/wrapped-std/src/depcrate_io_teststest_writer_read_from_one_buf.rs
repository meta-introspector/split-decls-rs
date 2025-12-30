// Generated macro for test_writer_read_from_one_buf (function)
macro_rules! Depcrate_io_teststest_writer_read_from_one_buf {
() => {
// Module: crate::io::tests
// Provides: {"test_writer_read_from_one_buf"}
// Dependencies: {}
# [test] fn test_writer_read_from_one_buf () { let mut writer = test_writer (1 , 2) ; assert_eq ! (writer . write (& []) . unwrap () , 0) ; assert_eq ! (writer . write_vectored (& []) . unwrap () , 0) ; assert_eq ! (writer . write (& [1 , 1 , 1]) . unwrap () , 2) ; let bufs = & [IoSlice :: new (& [2 , 2 , 2])] ; assert_eq ! (writer . write_vectored (bufs) . unwrap () , 2) ; let bufs = & [IoSlice :: new (& [3]) , IoSlice :: new (& [4 , 4])] ; assert_eq ! (writer . write_vectored (bufs) . unwrap () , 1) ; assert_eq ! (writer . written , & [1 , 1 , 2 , 2 , 3]) ; }
};
}
