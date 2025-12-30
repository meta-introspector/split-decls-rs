// Generated macro for read_buf_data_and_error_read_to_end (function)
macro_rules! Depcrate_io_testsread_buf_data_and_error_read_to_end {
() => {
// Module: crate::io::tests
// Provides: {"read_buf_data_and_error_read_to_end"}
// Dependencies: {}
# [test] fn read_buf_data_and_error_read_to_end () { let mut r = DataAndErrorReader (& [4 , 5 , 6]) ; let mut v = Vec :: with_capacity (200) ; assert ! (r . read_to_end (& mut v) . is_err ()) ; assert_eq ! (v , & [4 , 5 , 6]) ; }
};
}
