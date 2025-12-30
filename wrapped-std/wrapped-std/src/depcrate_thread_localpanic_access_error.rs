// Generated macro for panic_access_error (function)
macro_rules! Depcrate_thread_localpanic_access_error {
() => {
// Module: crate::thread::local
// Provides: {"panic_access_error"}
// Dependencies: {}
# [cfg_attr (not (feature = "panic_immediate_abort") , inline (never))] # [track_caller] # [cold] fn panic_access_error (err : AccessError) -> ! { panic ! ("cannot access a Thread Local Storage value during or after destruction: {err:?}") }
};
}
