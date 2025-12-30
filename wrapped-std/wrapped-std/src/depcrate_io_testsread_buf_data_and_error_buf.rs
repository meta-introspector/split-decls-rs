// Generated macro for read_buf_data_and_error_buf (function)
macro_rules! Depcrate_io_testsread_buf_data_and_error_buf {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_data_and_error_buf"}
// Dependencies: {}
# [test] fn read_buf_data_and_error_buf () { let mut r = BufReader :: new (DataAndErrorReader (& [4 , 5 , 6])) ; assert ! (r . fill_buf () . is_err ()) ; assert_eq ! (r . fill_buf () . unwrap () , & [4 , 5 , 6]) ; }
};
}
