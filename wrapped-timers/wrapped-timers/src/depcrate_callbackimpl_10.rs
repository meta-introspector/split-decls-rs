// Generated macro for impl_10 (impl)
macro_rules! Depcrate_callbackimpl_10 {
() => {
// Module: crate::callback
// Provides: {"impl_10"}
// Dependencies: {}
impl Drop for Interval { # [doc = " Disposes of the interval, dually cancelling this interval by calling"] # [doc = " `clearInterval` directly."] fn drop (& mut self) { if let Some (id) = self . id . take () { clear_interval (id) ; } } }
};
}
