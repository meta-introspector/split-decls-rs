// Generated macro for remove_dir_all_recursive (function)
macro_rules! Depcrate_sys_fs_commonremove_dir_all_recursive {
() => {
// Module: crate::sys::fs::common
// Provides: {"remove_dir_all_recursive"}
// Dependencies: {}
fn remove_dir_all_recursive (path : & Path) -> io :: Result < () > { for child in fs :: read_dir (path) ? { let result : io :: Result < () > = try { let child = child ? ; if child . file_type () ? . is_dir () { remove_dir_all_recursive (& child . path ()) ? ; } else { fs :: remove_file (& child . path ()) ? ; } } ; if let Err (err) = & result && err . kind () != io :: ErrorKind :: NotFound { return result ; } } ignore_notfound (fs :: remove_dir (path)) }
};
}
