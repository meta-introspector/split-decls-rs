// Generated macro for remove_dir (function)
macro_rules! Depcrate_sys_fsremove_dir {
() => {
// Module: crate::sys::fs
// Provides: {"remove_dir"}
// Dependencies: {}
pub fn remove_dir (path : & Path) -> io :: Result < () > { with_native_path (path , & imp :: rmdir) }
};
}
