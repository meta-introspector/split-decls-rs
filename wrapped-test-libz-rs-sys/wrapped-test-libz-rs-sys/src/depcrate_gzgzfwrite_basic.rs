// Generated macro for gzfwrite_basic (function)
macro_rules! Depcrate_gzgzfwrite_basic {
() => {
// Module: crate::gz
// Provides: {"gzfwrite_basic"}
// Dependencies: {}
# [test] fn gzfwrite_basic () { let temp_dir_path = temp_base () ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; let file_name = path (temp_path , "output") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("wT") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzfwrite (b"test" . as_ptr () . cast ::< c_void > () , 4 , 1 , file) } , 1) ; assert_eq ! (unsafe { gzfwrite (b" of gzfwrite..." . as_ptr () . cast ::< c_void > () , 4 , 3 , file) } , 3) ; assert_eq ! (unsafe { gzfread (ptr :: null_mut () , 1 , 1 , file) } , 0) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let mode = binary_mode (libc :: O_RDONLY) ; let fd = unsafe { libc :: open (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , mode) } ; assert_ne ! (fd , - 1) ; const EXPECTED : & [u8] = b"test of gzfwrite" ; let mut buf = [0u8 ; EXPECTED . len () + 1] ; let ret = unsafe { libc :: read (fd , buf . as_mut_ptr () . cast () , buf . len () as _) } ; assert_eq ! (ret , EXPECTED . len () as _) ; assert_eq ! (& buf [.. EXPECTED . len ()] , EXPECTED) ; assert_eq ! (unsafe { libc :: close (fd) } , 0) ; }
};
}
