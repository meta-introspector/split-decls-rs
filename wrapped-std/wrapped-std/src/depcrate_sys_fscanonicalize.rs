// Generated macro for canonicalize (function)
macro_rules! Depcrate_sys_fscanonicalize {
() => {
// Module: crate::sys::fs
// Provides: {"canonicalize"}
// Dependencies: {}
pub fn canonicalize (path : & Path) -> io :: Result < PathBuf > { with_native_path (path , & imp :: canonicalize) }
};
}
