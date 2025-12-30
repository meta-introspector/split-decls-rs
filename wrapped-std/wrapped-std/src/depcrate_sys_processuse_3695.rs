// Generated macro for use_3695 (pub_use)
macro_rules! Depcrate_sys_processuse_3695 {
() => {
// Module: crate::sys::process
// Provides: {"use_3695"}
// Dependencies: {}
# [cfg (not (any (all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "nuttx"))) , target_os = "windows" ,)))] pub use imp :: output ;
};
}
