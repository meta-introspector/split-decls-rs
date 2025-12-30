// Generated macro for copy (function)
macro_rules! Depcrate_sys_fscopy {
() => {
// Module: crate::sys::fs
// Provides: {"copy"}
// Dependencies: {}
pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { # [cfg (not (windows))] return imp :: copy (from , to) ; # [cfg (windows)] with_native_path (from , & | from | with_native_path (to , & | to | imp :: copy (from , to))) }
};
}
