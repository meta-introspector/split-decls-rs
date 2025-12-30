// Generated macro for gzgetc_basic (function)
macro_rules! Depcrate_gzgzgetc_basic {
() => {
// Module: crate::gz
// Provides: {"gzgetc_basic"}
// Dependencies: {}
# [test] fn gzgetc_basic () { for gzgetc_fn in [| x | unsafe { gzgetc (x) } , | x | unsafe { gzgetc_ (x) }] { let file_name = crate_path ("src/test-data/text.gz") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzbuffer (file , 8) } , 0) ; const EXPECTED : & str = "gzip\nexample data\nfor tests" ; let mut content = String :: with_capacity (EXPECTED . len ()) ; for _ in 0 .. EXPECTED . len () { let ch = gzgetc_fn (file) ; assert_ne ! (ch , - 1) ; content . push (ch as u8 as char) ; } assert_eq ! (gzgetc_fn (file) , - 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; assert_eq ! (content . as_str () , EXPECTED) ; } }
};
}
