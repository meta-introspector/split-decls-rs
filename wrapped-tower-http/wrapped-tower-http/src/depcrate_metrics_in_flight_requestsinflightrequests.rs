// Generated macro for InFlightRequests (struct)
macro_rules! Depcrate_metrics_in_flight_requestsInFlightRequests {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"InFlightRequests"}
// Dependencies: {}
# [doc = " Middleware that counts the number of in-flight requests."] # [doc = ""] # [doc = " See the [module docs](crate::metrics::in_flight_requests) for more details."] # [derive (Clone , Debug)] pub struct InFlightRequests < S > { inner : S , counter : InFlightRequestsCounter , }
};
}
