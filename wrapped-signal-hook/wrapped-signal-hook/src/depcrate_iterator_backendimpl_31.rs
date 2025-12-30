// Generated macro for impl_31 (impl)
macro_rules! Depcrate_iterator_backendimpl_31 {
() => {
// Module: crate::iterator::backend
// Provides: {"impl_31"}
// Dependencies: {}
impl Drop for DeliveryState { fn drop (& mut self) { let lock = self . registered_signal_ids . lock () . unwrap () ; for id in lock . iter () . filter_map (| s | * s) { crate :: low_level :: unregister (id) ; } } }
};
}
