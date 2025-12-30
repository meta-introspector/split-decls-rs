// Generated macro for gzoffset_gztell_write (function)
macro_rules! Depcrate_gzgzoffset_gztell_write {
() => {
// Module: crate::gz
// Provides: {"gzoffset_gztell_write"}
// Dependencies: {}
# [test] fn gzoffset_gztell_write () { let temp_dir_path = temp_base () ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; let file_name = path (temp_path , "output.gz") ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("w") . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; const BUFFER_SIZE : usize = 2048 ; assert_eq ! (unsafe { gzbuffer (file , BUFFER_SIZE as _) } , 0) ; assert_eq ! (unsafe { gztell (file) } , 0) ; assert_eq ! (unsafe { gzoffset (file) } , 0) ; let buf = [0u8 ; 1024] ; assert_eq ! (unsafe { gzwrite (file , buf . as_ptr () . cast ::< c_void > () , buf . len () as _) } , buf . len () as _) ; assert_eq ! (unsafe { gztell (file) } , buf . len () as _) ; assert_eq ! (unsafe { gzoffset (file) } , 0) ; assert_eq ! (unsafe { gzflush (file , Z_SYNC_FLUSH) } , Z_OK) ; assert_eq ! (unsafe { gztell (file) } , buf . len () as _) ; assert ! (unsafe { gzoffset (file) } > 0) ; let size = file_size (& file_name) . unwrap () ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let file = unsafe { gzopen (CString :: new (file_name . as_str ()) . unwrap () . as_ptr () , CString :: new ("a") . unwrap () . as_ptr () ,) } ; assert_eq ! (unsafe { gztell (file) } , 0) ; assert_eq ! (unsafe { gzoffset (file) } , (size + 10) as _) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; }
};
}
