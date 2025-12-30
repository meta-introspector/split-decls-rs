// Generated macro for impl_747 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_747 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_747"}
// Dependencies: {}
impl Drop for IncrementGuard { fn drop (& mut self) { self . count . fetch_sub (1 , Ordering :: Relaxed) ; } }
};
}
