// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_sys_fs_commonremove_dir_all {
() => {
// Module: crate::sys::fs::common
// Provides: {"remove_dir_all"}
// Dependencies: {}
pub fn remove_dir_all (path : & Path) -> io :: Result < () > { let filetype = fs :: symlink_metadata (path) ? . file_type () ; if filetype . is_symlink () { fs :: remove_file (path) } else { remove_dir_all_recursive (path) } }
};
}
