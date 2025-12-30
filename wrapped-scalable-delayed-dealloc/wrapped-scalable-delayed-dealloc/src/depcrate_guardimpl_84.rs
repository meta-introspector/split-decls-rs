// Generated macro for impl_84 (impl)
macro_rules! Depcrate_guardimpl_84 {
() => {
// Module: crate::guard
// Provides: {"impl_84"}
// Dependencies: {}
impl Drop for Guard { # [inline] fn drop (& mut self) { Collector :: end_guard (self . collector_ptr . as_ptr ()) ; } }
};
}
