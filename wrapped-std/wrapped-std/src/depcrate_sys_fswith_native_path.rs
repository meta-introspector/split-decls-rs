// Generated macro for with_native_path (function)
macro_rules! Depcrate_sys_fswith_native_path {
() => {
// Module: crate::sys::fs
// Provides: {"with_native_path"}
// Dependencies: {}
# [cfg (not (any (target_family = "unix" , target_os = "windows")))] # [inline] pub fn with_native_path < T > (path : & Path , f : & dyn Fn (& Path) -> io :: Result < T >) -> io :: Result < T > { f (path) }
};
}
