// Generated macro for gz_direct_read (function)
macro_rules! Depcrate_gzgz_direct_read {
() => {
// Module: crate::gz
// Provides: {"gz_direct_read"}
// Dependencies: {}
# [test] fn gz_direct_read () { assert_eq ! (unsafe { gzdirect (ptr :: null_mut ()) } , 0) ; let file = unsafe { gzopen (CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzdirect (file) } , 0) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let file = unsafe { gzopen (CString :: new (crate_path ("src/test-data/issue-169.js")) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzdirect (file) } , 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzdirect (file) } , 1) ; let mut err = Z_OK ; let msg = unsafe { gzerror (file , & mut err as * mut c_int) } ; assert ! (! msg . is_null ()) ; assert_eq ! (err , Z_ERRNO) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
