// Generated macro for test_open (macro)
macro_rules! Depcrate_gztest_open {
() => {
// Module: crate::gz
// Provides: {"test_open"}
// Dependencies: {}
macro_rules ! test_open { ($ path : expr , $ mode : expr , $ should_succeed : expr) => { let cpath = CString :: new ($ path) . unwrap () ; let cmode = CString :: new ($ mode) . unwrap () ; let handle = unsafe { gzopen (cpath . as_ptr () , cmode . as_ptr ()) } ; assert_eq ! ($ should_succeed , ! handle . is_null () , "gzopen({}, {})" , $ path , $ mode) ; if ! handle . is_null () { assert_eq ! (unsafe { gzclose (handle) } , Z_OK , "gzclose({}) error" , $ path) ; } } ; }
};
}
