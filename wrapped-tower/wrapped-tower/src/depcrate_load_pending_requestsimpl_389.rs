// Generated macro for impl_389 (impl)
macro_rules! Depcrate_load_pending_requestsimpl_389 {
() => {
// Module: crate::load::pending_requests
// Provides: {"impl_389"}
// Dependencies: {}
impl < S , C > Load for PendingRequests < S , C > { type Metric = Count ; fn load (& self) -> Count { Count (self . ref_count . ref_count () - 1) } }
};
}
