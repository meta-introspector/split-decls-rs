// Generated macro for test_fdopen (macro)
macro_rules! Depcrate_gztest_fdopen {
() => {
// Module: crate::gz
// Provides: {"test_fdopen"}
// Dependencies: {}
macro_rules ! test_fdopen { ($ fd : expr , $ mode : expr , $ should_succeed : expr) => { let cmode = CString :: new ($ mode) . unwrap () ; let handle = unsafe { gzdopen ($ fd , cmode . as_ptr ()) } ; assert_eq ! ($ should_succeed , ! handle . is_null () , "gzdopen({}, {})" , $ fd , $ mode) ; if ! handle . is_null () { assert_eq ! (unsafe { gzclose (handle) } , Z_OK , "gzclose({}) error" , $ fd) ; } } ; }
};
}
