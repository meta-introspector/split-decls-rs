// Generated macro for gzputs_error (function)
macro_rules! Depcrate_gzgzputs_error {
() => {
// Module: crate::gz
// Provides: {"gzputs_error"}
// Dependencies: {}
# [test] fn gzputs_error () { const CONTENT : & [u8] = b"example\0" ; assert_eq ! (unsafe { gzputs (ptr :: null_mut () , CONTENT . as_ptr () . cast ::< c_char > ()) } , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzputs (ptr :: null_mut () , CONTENT . as_ptr () . cast ::< c_char > ()) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
