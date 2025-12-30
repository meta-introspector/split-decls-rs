// Generated macro for read_buf_data_and_error_take (function)
macro_rules! Depcrate_io_testsread_buf_data_and_error_take {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_data_and_error_take"}
// Dependencies: {}
# [test] fn read_buf_data_and_error_take () { let mut buf = [0 ; 64] ; let mut buf = io :: BorrowedBuf :: from (buf . as_mut_slice ()) ; let mut r = DataAndErrorReader (& [4 , 5 , 6]) . take (1) ; assert ! (r . read_buf (buf . unfilled ()) . is_err ()) ; assert_eq ! (buf . filled () , & [4]) ; assert ! (r . read_buf (buf . unfilled ()) . is_ok ()) ; assert_eq ! (buf . filled () , & [4]) ; assert_eq ! (r . get_ref () . 0 , & [5 , 6]) ; }
};
}
