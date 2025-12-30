// Generated macro for PanicGuard (struct)
macro_rules! Depcrate_threadPanicGuard {
() => {
// Module: crate::thread
// Provides: {"PanicGuard"}
// Dependencies: {}
# [doc = " Used to ensure that `park` and `park_timeout` do not unwind, as that can"] # [doc = " cause undefined behavior if not handled correctly (see #102398 for context)."] struct PanicGuard ;
};
}
