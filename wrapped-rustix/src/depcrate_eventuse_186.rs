// Generated macro for use_186 (pub_use)
macro_rules! Depcrate_eventuse_186 {
() => {
// Module: crate::event
// Provides: {"use_186"}
// Dependencies: {}
# [cfg (any (linux_kernel , target_os = "freebsd" , target_os = "illumos" , target_os = "espidf"))] pub use eventfd :: { eventfd , EventfdFlags } ;
};
}
