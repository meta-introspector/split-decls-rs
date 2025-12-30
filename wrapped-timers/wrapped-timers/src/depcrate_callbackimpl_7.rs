// Generated macro for impl_7 (impl)
macro_rules! Depcrate_callbackimpl_7 {
() => {
// Module: crate::callback
// Provides: {"impl_7"}
// Dependencies: {}
impl Drop for Timeout { # [doc = " Disposes of the timeout, dually cancelling this timeout by calling"] # [doc = " `clearTimeout` directly."] fn drop (& mut self) { if let Some (id) = self . id . take () { clear_timeout (id) ; } } }
};
}
