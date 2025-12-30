// Generated macro for read_dir (function)
macro_rules! Depcrate_sys_fsread_dir {
() => {
// Module: crate::sys::fs
// Provides: {"read_dir"}
// Dependencies: {}
pub fn read_dir (path : & Path) -> io :: Result < ReadDir > { imp :: readdir (path) }
};
}
