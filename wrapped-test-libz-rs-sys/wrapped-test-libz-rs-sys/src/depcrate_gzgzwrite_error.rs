// Generated macro for gzwrite_error (function)
macro_rules! Depcrate_gzgzwrite_error {
() => {
// Module: crate::gz
// Provides: {"gzwrite_error"}
// Dependencies: {}
# [test] fn gzwrite_error () { const STRING1 : & [u8] = b"sample data" ; assert_eq ! (unsafe { gzwrite (ptr :: null_mut () , STRING1 . as_ptr () . cast ::< c_void > () , STRING1 . len () as _ ,) } , 0) ; let file = unsafe { gzopen (CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert_eq ! (unsafe { gzwrite (file , STRING1 . as_ptr () . cast ::< c_void > () , STRING1 . len () as _) } , 0) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let len = c_int :: MAX as c_uint + 1 ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzwrite (file , STRING1 . as_ptr () . cast ::< c_void > () , len) } , 0) ; let mut err = Z_OK ; assert ! (! unsafe { gzerror (file , & mut err as * mut c_int) } . is_null ()) ; assert_eq ! (err , Z_DATA_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
