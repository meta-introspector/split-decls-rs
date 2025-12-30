// Generated macro for PID (type)
macro_rules! Depcrate_c_interfacePID {
() => {
// Module: crate::c_interface
// Provides: {"PID"}
// Dependencies: {}
# [doc = " other platforms, use libc::pid_t"] # [cfg (not (target_os = "windows"))] pub type PID = libc :: pid_t ;
};
}
