// Generated macro for gzungetc_error (function)
macro_rules! Depcrate_gzgzungetc_error {
() => {
// Module: crate::gz
// Provides: {"gzungetc_error"}
// Dependencies: {}
# [test] fn gzungetc_error () { assert_eq ! (unsafe { gzungetc ('*' as c_int , ptr :: null_mut ()) } , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert_eq ! (unsafe { gzungetc ('*' as c_int , file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file_name = crate_path ("src/test-data/text.gz") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzungetc (- 1 as c_int , file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
