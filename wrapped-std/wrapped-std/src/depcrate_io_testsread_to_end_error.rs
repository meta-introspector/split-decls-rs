// Generated macro for read_to_end_error (function)
macro_rules! Depcrate_io_testsread_to_end_error {
() => {
// Module: crate::io::tests
// Provides: {"read_to_end_error"}
// Dependencies: {}
# [test] fn read_to_end_error () { struct ErrorReader ; impl Read for ErrorReader { fn read (& mut self , _buf : & mut [u8]) -> io :: Result < usize > { Err (io :: Error :: other ("error")) } } let mut r = [4 , 5 , 6] . chain (ErrorReader) ; let mut v = Vec :: with_capacity (200) ; assert ! (r . read_to_end (& mut v) . is_err ()) ; assert_eq ! (v , & [4 , 5 , 6]) ; }
};
}
