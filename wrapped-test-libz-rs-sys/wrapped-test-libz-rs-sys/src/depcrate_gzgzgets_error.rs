// Generated macro for gzgets_error (function)
macro_rules! Depcrate_gzgzgets_error {
() => {
// Module: crate::gz
// Provides: {"gzgets_error"}
// Dependencies: {}
# [test] fn gzgets_error () { let mut buf = [0 as c_char ; 16] ; assert ! (unsafe { gzgets (ptr :: null_mut () , buf . as_mut_ptr () , buf . len () as _) } . is_null ()) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (unsafe { gzgets (ptr :: null_mut () , buf . as_mut_ptr () , buf . len () as _) } . is_null ()) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (unsafe { gzgets (ptr :: null_mut () , buf . as_mut_ptr () , buf . len () as _) } . is_null ()) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file_name = crate_path ("src/test-data/issue-109.gz") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert ! (unsafe { gzgets (ptr :: null_mut () , ptr :: null_mut () , 1) } . is_null ()) ; assert ! (unsafe { gzgets (ptr :: null_mut () , buf . as_mut_ptr () , 0) } . is_null ()) ; assert ! (unsafe { gzgets (ptr :: null_mut () , buf . as_mut_ptr () , - 1) } . is_null ()) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
