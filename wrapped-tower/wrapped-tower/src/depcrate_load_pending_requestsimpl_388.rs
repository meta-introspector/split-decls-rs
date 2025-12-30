// Generated macro for impl_388 (impl)
macro_rules! Depcrate_load_pending_requestsimpl_388 {
() => {
// Module: crate::load::pending_requests
// Provides: {"impl_388"}
// Dependencies: {}
impl < S , C > PendingRequests < S , C > { # [doc = " Wraps an `S`-typed service so that its load is tracked by the number of pending requests."] pub fn new (service : S , completion : C) -> Self { Self { service , completion , ref_count : RefCount :: default () , } } fn handle (& self) -> Handle { Handle (self . ref_count . clone ()) } }
};
}
