// Generated macro for exists (function)
macro_rules! Depcrate_sys_fsexists {
() => {
// Module: crate::sys::fs
// Provides: {"exists"}
// Dependencies: {}
pub fn exists (path : & Path) -> io :: Result < bool > { # [cfg (not (windows))] return imp :: exists (path) ; # [cfg (windows)] with_native_path (path , & imp :: exists) }
};
}
