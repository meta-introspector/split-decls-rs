// Generated macro for gzgetc_error (function)
macro_rules! Depcrate_gzgzgetc_error {
() => {
// Module: crate::gz
// Provides: {"gzgetc_error"}
// Dependencies: {}
# [test] fn gzgetc_error () { for gzgetc_fn in [| x | unsafe { gzgetc (x) } , | x | unsafe { gzgetc_ (x) }] { assert_eq ! (gzgetc_fn (ptr :: null_mut ()) , - 1) ; let file = unsafe { gzdopen (- 2 , CString :: new ("w") . unwrap () . as_ptr ()) } ; assert_eq ! (gzgetc_fn (file) , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; let file = unsafe { gzdopen (- 2 , CString :: new ("r") . unwrap () . as_ptr ()) } ; assert_eq ! (gzgetc_fn (file) , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; } }
};
}
