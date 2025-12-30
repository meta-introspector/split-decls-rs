// Generated macro for gzsetparams_error (function)
macro_rules! Depcrate_gzgzsetparams_error {
() => {
// Module: crate::gz
// Provides: {"gzsetparams_error"}
// Dependencies: {}
# [test] fn gzsetparams_error () { assert_eq ! (unsafe { gzsetparams (ptr :: null_mut () , 1 , 0) } , Z_STREAM_ERROR) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzsetparams (file , 1 , 0) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("wT") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzsetparams (file , 1 , 0) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzsetparams (file , 1 , 0) } , Z_OK) ; assert_eq ! (unsafe { gzsetparams (file , 1 , - 1) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; const CONTENT : & [u8] = b"0123456789" ; assert_eq ! (unsafe { gzwrite (file , CONTENT . as_ptr () . cast ::< c_void > () , CONTENT . len () as _) } , CONTENT . len () as _) ; assert_eq ! (unsafe { gzsetparams (file , 1 , 2) } , Z_ERRNO) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
