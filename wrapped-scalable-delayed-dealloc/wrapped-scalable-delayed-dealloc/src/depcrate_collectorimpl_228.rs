// Generated macro for impl_228 (impl)
macro_rules! Depcrate_collectorimpl_228 {
() => {
// Module: crate::collector
// Provides: {"impl_228"}
// Dependencies: {}
impl Drop for Collector { # [inline] fn drop (& mut self) { let collector_ptr = addr_of_mut ! (* self) ; Self :: clear_for_drop (collector_ptr) ; } }
};
}
