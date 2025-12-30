// Generated macro for gz_error_access (function)
macro_rules! Depcrate_gzgz_error_access {
() => {
// Module: crate::gz
// Provides: {"gz_error_access"}
// Dependencies: {}
# [test] fn gz_error_access () { const UNSET_ERRNO : c_int = - 12345 ; assert ! (unsafe { gzerror (ptr :: null_mut ::< gzFile_s > () , ptr :: null_mut ()) . is_null () }) ; let mut gz_errno : c_int = UNSET_ERRNO ; assert ! (unsafe { gzerror (ptr :: null_mut ::< gzFile_s > () , & mut gz_errno as * mut c_int) . is_null () }) ; assert_eq ! (gz_errno , UNSET_ERRNO) ; let path = CString :: new (crate_path ("src/test-data/issue-109.gz")) . unwrap () ; let mode = CString :: new ("r") . unwrap () ; let handle = unsafe { gzopen64 (path . as_ptr () , mode . as_ptr ()) } ; assert ! (! handle . is_null ()) ; let mut gz_errno : c_int = UNSET_ERRNO ; let err = unsafe { gzerror (handle , & mut gz_errno as * mut c_int) } ; assert ! (! err . is_null ()) ; assert_eq ! (unsafe { * err } , 0 as c_char) ; assert_eq ! (unsafe { gzclose (handle) } , Z_OK) ; }
};
}
