// Generated macro for gzclose_error (function)
macro_rules! Depcrate_gzgzclose_error {
() => {
// Module: crate::gz
// Provides: {"gzclose_error"}
// Dependencies: {}
# [test] fn gzclose_error () { assert_eq ! (unsafe { gzclose_r (ptr :: null_mut ()) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose_w (ptr :: null_mut ()) } , Z_STREAM_ERROR) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzclose_r (file) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose_w (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzclose_w (file) } , Z_STREAM_ERROR) ; assert_eq ! (unsafe { gzclose_r (file) } , Z_ERRNO) ; }
};
}
