// Generated macro for read_link (function)
macro_rules! Depcrate_sys_fsread_link {
() => {
// Module: crate::sys::fs
// Provides: {"read_link"}
// Dependencies: {}
pub fn read_link (path : & Path) -> io :: Result < PathBuf > { with_native_path (path , & imp :: readlink) }
};
}
