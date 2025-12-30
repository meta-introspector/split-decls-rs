// Generated macro for remove_dir_all (function)
macro_rules! Depcrate_sys_fsremove_dir_all {
() => {
// Module: crate::sys::fs
// Provides: {"remove_dir_all"}
// Dependencies: {}
pub fn remove_dir_all (path : & Path) -> io :: Result < () > { # [cfg (not (windows))] return imp :: remove_dir_all (path) ; # [cfg (windows)] with_native_path (path , & imp :: remove_dir_all) }
};
}
