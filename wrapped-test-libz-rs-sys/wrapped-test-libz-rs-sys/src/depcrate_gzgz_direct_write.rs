// Generated macro for gz_direct_write (function)
macro_rules! Depcrate_gzgz_direct_write {
() => {
// Module: crate::gz
// Provides: {"gz_direct_write"}
// Dependencies: {}
# [test] fn gz_direct_write () { let temp_dir_path = if cfg ! (target_os = "wasi") { std :: path :: PathBuf :: from ("/tmp/") } else { std :: env :: temp_dir () } ; let temp_dir = tempfile :: TempDir :: new_in (temp_dir_path) . unwrap () ; let temp_path = temp_dir . path () ; for mode in ["w" , "a"] { let file = unsafe { gzopen (CString :: new (path (temp_path , "compressed.gz")) . unwrap () . as_ptr () , CString :: new (mode) . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzdirect (file) } , 0) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; let file = unsafe { gzopen (CString :: new (path (temp_path , "direct.gz")) . unwrap () . as_ptr () , CString :: new ("T" . to_owned () + mode) . unwrap () . as_ptr () ,) } ; assert ! (! file . is_null ()) ; assert_eq ! (unsafe { gzdirect (file) } , 1) ; assert_eq ! (unsafe { gzclose (file) } , Z_OK) ; } }
};
}
