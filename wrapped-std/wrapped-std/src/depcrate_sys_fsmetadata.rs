// Generated macro for metadata (function)
macro_rules! Depcrate_sys_fsmetadata {
() => {
// Module: crate::sys::fs
// Provides: {"metadata"}
// Dependencies: {}
pub fn metadata (path : & Path) -> io :: Result < FileAttr > { with_native_path (path , & imp :: stat) }
};
}
