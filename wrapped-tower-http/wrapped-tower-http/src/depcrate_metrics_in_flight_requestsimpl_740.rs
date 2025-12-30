// Generated macro for impl_740 (impl)
macro_rules! Depcrate_metrics_in_flight_requestsimpl_740 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"impl_740"}
// Dependencies: {}
impl InFlightRequestsLayer { # [doc = " Create a new `InFlightRequestsLayer` and its associated counter."] pub fn pair () -> (Self , InFlightRequestsCounter) { let counter = InFlightRequestsCounter :: new () ; let layer = Self :: new (counter . clone ()) ; (layer , counter) } # [doc = " Create a new `InFlightRequestsLayer` that will update the given counter."] pub fn new (counter : InFlightRequestsCounter) -> Self { Self { counter } } }
};
}
