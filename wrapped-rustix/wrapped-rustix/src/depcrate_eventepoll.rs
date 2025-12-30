// Generated macro for epoll (module)
macro_rules! Depcrate_eventepoll {
() => {
// Module: crate::event
// Provides: {"epoll"}
// Dependencies: {}
# [cfg (any (linux_kernel , target_os = "illumos" , target_os = "redox"))] pub mod epoll ;
};
}
