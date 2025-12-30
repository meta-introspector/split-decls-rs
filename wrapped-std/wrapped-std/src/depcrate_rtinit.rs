// Generated macro for init (function)
macro_rules! Depcrate_rtinit {
() => {
// Module: crate::rt
// Provides: {"init"}
// Dependencies: {}
# [cfg_attr (test , allow (dead_code))] unsafe fn init (argc : isize , argv : * const * const u8 , sigpipe : u8) { # [cfg_attr (target_os = "teeos" , allow (unused_unsafe))] unsafe { sys :: init (argc , argv , sigpipe) } ; unsafe { main_thread :: set (thread :: current_id ()) } ; }
};
}
