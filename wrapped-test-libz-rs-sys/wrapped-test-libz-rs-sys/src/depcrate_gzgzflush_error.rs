// Generated macro for gzflush_error (function)
macro_rules! Depcrate_gzgzflush_error {
() => {
// Module: crate::gz
// Provides: {"gzflush_error"}
// Dependencies: {}
# [test] fn gzflush_error () { assert_eq ! (unsafe { gzflush (ptr :: null_mut () , Z_NO_FLUSH) } , Z_STREAM_ERROR) ; let file = unsafe { gzopen (CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert_eq ! (unsafe { gzflush (file , Z_NO_FLUSH) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzflush (file , - 1) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzflush (file , Z_FINISH + 1) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
