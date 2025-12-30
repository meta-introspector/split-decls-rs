// Generated macro for remove_file (function)
macro_rules! Depcrate_sys_fsremove_file {
() => {
// Module: crate::sys::fs
// Provides: {"remove_file"}
// Dependencies: {}
pub fn remove_file (path : & Path) -> io :: Result < () > { with_native_path (path , & imp :: unlink) }
};
}
