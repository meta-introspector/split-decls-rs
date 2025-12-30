// Generated macro for InFlightRequestsCounter (struct)
macro_rules! Depcrate_metrics_in_flight_requestsInFlightRequestsCounter {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"InFlightRequestsCounter"}
// Dependencies: {}
# [doc = " An atomic counter that keeps track of the number of in-flight requests."] # [doc = ""] # [doc = " This will normally combined with [`InFlightRequestsLayer`] or [`InFlightRequests`] which will"] # [doc = " update the counter as requests arrive."] # [derive (Debug , Clone , Default)] pub struct InFlightRequestsCounter { count : Arc < AtomicUsize > , }
};
}
