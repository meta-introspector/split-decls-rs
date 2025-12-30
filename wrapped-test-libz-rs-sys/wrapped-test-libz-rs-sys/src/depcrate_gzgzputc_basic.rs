// Generated macro for gzputc_basic (function)
macro_rules! Depcrate_gzgzputc_basic {
() => {
// Module: crate::gz
// Provides: {"gzputc_basic"}
// Dependencies: {}
# [test] fn gzputc_basic () { let temp_dir_path = temp_base () ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; let file_name = path (temp_path , "output") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("wT") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzbuffer (file , 8) } , 0) ; const CONTENT : & [u8] = b"sample text to test gzputc implementation" ; for c in CONTENT { assert_eq ! (unsafe { gzputc (file , * c as _) } , * c as _) ; } assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let mode = binary_mode (libc :: O_RDONLY) ; let fd = unsafe { libc :: open (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , mode) } ; assert_ne ! (fd , - 1) ; let mut buf = [0u8 ; CONTENT . len () + 1] ; let bytes_read = unsafe { libc :: read (fd , buf . as_mut_ptr () as * mut c_void , buf . len () as _) } ; assert_ne ! (bytes_read , - 1) ; assert_eq ! (& buf [.. bytes_read as usize] , CONTENT) ; assert_eq ! (unsafe { libc :: close (fd) } , 0) ; }
};
}
