// Generated macro for macro_749 (macro)
macro_rules! Depcrate_metrics_in_flight_requestsmacro_749 {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"macro_749"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`InFlightRequests`]."] pub struct ResponseFuture < F > { # [pin] inner : F , guard : Option < IncrementGuard >, } }
};
}
