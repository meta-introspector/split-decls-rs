// Generated macro for symlink_metadata (function)
macro_rules! Depcrate_sys_fssymlink_metadata {
() => {
// Module: crate::sys::fs
// Provides: {"symlink_metadata"}
// Dependencies: {}
pub fn symlink_metadata (path : & Path) -> io :: Result < FileAttr > { with_native_path (path , & imp :: lstat) }
};
}
