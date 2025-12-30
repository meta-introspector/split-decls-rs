// Generated macro for gzputc_error (function)
macro_rules! Depcrate_gzgzputc_error {
() => {
// Module: crate::gz
// Provides: {"gzputc_error"}
// Dependencies: {}
# [test] fn gzputc_error () { assert_eq ! (unsafe { gzputc (ptr :: null_mut () , 1) } , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzputc (ptr :: null_mut () , 1) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("wT") . unwrap () . as_ptr ()) } ; const BUF_SIZE : usize = 10 ; assert_eq ! (unsafe { gzbuffer (file , BUF_SIZE as _) } , 0) ; for _ in 0 .. BUF_SIZE { assert_eq ! (unsafe { gzputc (file , 1) } , 1) ; } assert_eq ! (unsafe { gzputc (file , 1) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; }
};
}
