// Generated macro for impl_741 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_741 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_741"}
// Dependencies: {}
impl < S > Layer < S > for InFlightRequestsLayer { type Service = InFlightRequests < S > ; fn layer (& self , inner : S) -> Self :: Service { InFlightRequests { inner , counter : self . counter . clone () , } } }
};
}
