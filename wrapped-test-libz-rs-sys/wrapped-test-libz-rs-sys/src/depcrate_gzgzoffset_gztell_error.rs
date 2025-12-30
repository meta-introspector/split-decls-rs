// Generated macro for gzoffset_gztell_error (function)
macro_rules! Depcrate_gzgzoffset_gztell_error {
() => {
// Module: crate::gz
// Provides: {"gzoffset_gztell_error"}
// Dependencies: {}
# [test] fn gzoffset_gztell_error () { assert_eq ! (unsafe { gzoffset (ptr :: null_mut ()) } , - 1) ; assert_eq ! (unsafe { gztell (ptr :: null_mut ()) } , - 1) ; for mode in [b"r" , b"w" , b"a"] { let file = unsafe { gzdopen (- 2 , CString :: new (mode) . unwrap () . as_ptr ()) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gztell (file) } , 0) ; assert_eq ! (unsafe { gzoffset (file) } , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_ERRNO) ; } }
};
}
