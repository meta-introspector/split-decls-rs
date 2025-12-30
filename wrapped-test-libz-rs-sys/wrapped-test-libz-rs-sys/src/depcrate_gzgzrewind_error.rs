// Generated macro for gzrewind_error (function)
macro_rules! Depcrate_gzgzrewind_error {
() => {
// Module: crate::gz
// Provides: {"gzrewind_error"}
// Dependencies: {}
# [test] fn gzrewind_error () { assert_eq ! (unsafe { gzrewind (ptr :: null_mut ()) } , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzrewind (file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzrewind (file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; let mut buf = [0u8 ; 16] ; assert_eq ! (unsafe { gzread (file , buf . as_mut_ptr () . cast ::< c_void > () , buf . len () as _) } , - 1) ; assert_eq ! (unsafe { gzrewind (file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
