// Generated macro for gzfread_basic (function)
macro_rules! Depcrate_gzgzfread_basic {
() => {
// Module: crate::gz
// Provides: {"gzfread_basic"}
// Dependencies: {}
# [test] fn gzfread_basic () { let mut buf = [0u8 ; 32] ; let file_name = crate_path ("src/test-data/text.gz") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("r") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 4 , 5 , file) } , 5) ; assert_eq ! (& buf [.. 20] , b"gzip\nexample data\nfo") ; assert_eq ! (buf [20] , 0) ; assert_eq ! (unsafe { gzfread (ptr :: null_mut () , 1 , 1 , file) } , 0) ; let mut buf = [0u8 ; 32] ; assert_eq ! (unsafe { gzfread (buf . as_mut_ptr () . cast ::< c_void > () , 4 , 5 , file) } , 1) ; assert_eq ! (& buf [.. 4] , b"r te") ; assert_eq ! (& buf [4 .. 8] , b"sts\0") ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
