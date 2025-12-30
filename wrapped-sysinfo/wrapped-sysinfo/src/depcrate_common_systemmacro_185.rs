// Generated macro for macro_185 (macro)
macro_rules! Depcrate_common_systemmacro_185 {
() => {
// Module: crate::common::system
// Provides: {"macro_185"}
// Dependencies: {}
cfg_if ! { if # [cfg (all (not (feature = "unknown-ci") , any (target_os = "freebsd" , target_os = "linux" , target_os = "android" , target_os = "macos" , target_os = "ios" ,)))] { use libc :: pid_t ; pid_decl ! (pid_t) ; } else { pid_decl ! (usize) ; } }
};
}
