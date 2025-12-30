// Generated macro for use_2005 (pub_use)
macro_rules! Depcrate_timeuse_2005 {
() => {
// Module: crate::time
// Provides: {"use_2005"}
// Dependencies: {}
# [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "fuchsia" , target_os = "illumos" , target_os = "netbsd"))] pub use timerfd :: * ;
};
}
