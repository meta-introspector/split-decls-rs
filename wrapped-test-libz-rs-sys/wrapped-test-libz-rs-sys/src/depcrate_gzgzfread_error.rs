// Generated macro for gzfread_error (function)
macro_rules! Depcrate_gzgzfread_error {
() => {
// Module: crate::gz
// Provides: {"gzfread_error"}
// Dependencies: {}
# [test] fn gzfread_error () { let mut buf = [0u8 ; 10] ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 1 , 1 , ptr :: null_mut ()) } , 0) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 0 , 1 , file) } , 0) ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 1 , 0 , file) } , 0) ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , size_t :: MAX , 2 , file) } , 0) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 1 , 1 , file) } , 0) ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
