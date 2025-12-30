// Generated macro for impl_308 (impl)
macro_rules! Depcrate_thread_atomicsimpl_308 {
() => {
// Module: crate::thread::atomics
// Provides: {"impl_308"}
// Dependencies: {}
impl Thread { # [doc = " Registers the given `thread`."] fn register (thread : Self) { THREAD . with (| cell | cell . set (thread) . expect ("`Thread` already registered")) ; } }
};
}
