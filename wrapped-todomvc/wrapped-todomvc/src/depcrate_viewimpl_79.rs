// Generated macro for impl_79 (impl)
macro_rules! Depcrate_viewimpl_79 {
() => {
// Module: crate::view
// Provides: {"impl_79"}
// Dependencies: {}
impl Drop for View { fn drop (& mut self) { for callback in self . callbacks . drain (..) { callback . 0 . remove_event_listener_with_callback (callback . 1 . as_str () , callback . 2 . as_ref () . unchecked_ref () ,) . unwrap () ; } exit ("calling drop on view") ; } }
};
}
