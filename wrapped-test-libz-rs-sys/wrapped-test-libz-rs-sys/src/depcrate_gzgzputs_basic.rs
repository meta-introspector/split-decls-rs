// Generated macro for gzputs_basic (function)
macro_rules! Depcrate_gzgzputs_basic {
() => {
// Module: crate::gz
// Provides: {"gzputs_basic"}
// Dependencies: {}
# [test] fn gzputs_basic () { let temp_dir_path = temp_base () ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; let file_name = path (temp_path , "output") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("wT") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzbuffer (file , 8) } , 0) ; assert_eq ! (unsafe { gzputs (file , ptr :: null ()) } , - 1) ; const CONTENT : [& str ; 3] = ["zlib " , "" , "string larger than the buffer size"] ; for s in CONTENT { assert_eq ! (unsafe { gzputs (file , CString :: new (s) . unwrap () . as_ptr ()) } , s . len () as _) ; } assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; const EXPECTED : & str = "zlib string larger than the buffer size" ; let mode = binary_mode (libc :: O_RDONLY) ; let fd = unsafe { libc :: open (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , mode) } ; assert_ne ! (fd , - 1) ; let mut buf = [0u8 ; EXPECTED . len () + 1] ; let bytes_read = unsafe { libc :: read (fd , buf . as_mut_ptr () as * mut c_void , buf . len () as _) } ; assert_ne ! (bytes_read , - 1) ; assert_eq ! (& buf [.. bytes_read as usize] , EXPECTED . as_bytes ()) ; assert_eq ! (unsafe { libc :: close (fd) } , 0) ; }
};
}
