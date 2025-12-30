// Generated macro for drop_current (function)
macro_rules! Depcrate_thread_currentdrop_current {
() => {
// Module: crate::thread::current
// Provides: {"drop_current"}
// Dependencies: {}
# [doc = " This should be run in [`crate::rt::thread_cleanup`] to reset the thread"] # [doc = " handle."] pub (crate) fn drop_current () { let current = CURRENT . get () ; if current > DESTROYED { unsafe { CURRENT . set (DESTROYED) ; drop (Thread :: from_raw (current)) ; } } }
};
}
