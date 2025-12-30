// Generated macro for thread_cleanup (function)
macro_rules! Depcrate_rtthread_cleanup {
() => {
// Module: crate::rt
// Provides: {"thread_cleanup"}
// Dependencies: {}
# [doc = " Clean up the thread-local runtime state. This *should* be run after all other"] # [doc = " code managed by the Rust runtime, but will not cause UB if that condition is"] # [doc = " not fulfilled. Also note that this function is not guaranteed to be run, but"] # [doc = " skipping it will cause leaks and therefore is to be avoided."] pub (crate) fn thread_cleanup () { panic :: catch_unwind (| | { crate :: thread :: drop_current () ; }) . unwrap_or_else (handle_rt_panic) ; }
};
}
