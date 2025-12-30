// Generated macro for current (function)
macro_rules! Depcrate_threadcurrent {
() => {
// Module: crate::thread
// Provides: {"current"}
// Dependencies: {}
# [doc = " See [`std::thread::current()`]."] # [must_use] pub fn current () -> Thread { THREAD . with (| cell | cell . get_or_init (Thread :: new) . clone ()) }
};
}
