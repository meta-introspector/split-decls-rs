// Generated macro for get_global (function)
macro_rules! Depcrate_dispatcherget_global {
() => {
// Module: crate::dispatcher
// Provides: {"get_global"}
// Dependencies: {}
# [inline] fn get_global () -> & 'static Dispatch { if GLOBAL_INIT . load (Ordering :: SeqCst) != INITIALIZED { return & NONE ; } unsafe { & * addr_of ! (GLOBAL_DISPATCH) } }
};
}
