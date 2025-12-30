// Generated macro for impl_743 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_743 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_743"}
// Dependencies: {}
impl < S > InFlightRequests < S > { # [doc = " Create a new `InFlightRequests` and its associated counter."] pub fn pair (inner : S) -> (Self , InFlightRequestsCounter) { let counter = InFlightRequestsCounter :: new () ; let service = Self :: new (inner , counter . clone ()) ; (service , counter) } # [doc = " Create a new `InFlightRequests` that will update the given counter."] pub fn new (inner : S , counter : InFlightRequestsCounter) -> Self { Self { inner , counter } } define_inner_service_accessors ! () ; }
};
}
