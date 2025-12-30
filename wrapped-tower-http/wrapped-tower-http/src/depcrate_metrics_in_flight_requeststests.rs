// Generated macro for tests (module)
macro_rules! Depcrate_metrics_in_flight_requeststests {
() => {
// Module: crate::metrics::in_flight_requests
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [allow (unused_imports)] use super :: * ; use crate :: test_helpers :: Body ; use http :: Request ; use tower :: { BoxError , ServiceBuilder } ; # [tokio :: test] async fn basic () { let (in_flight_requests_layer , counter) = InFlightRequestsLayer :: pair () ; let mut service = ServiceBuilder :: new () . layer (in_flight_requests_layer) . service_fn (echo) ; assert_eq ! (counter . get () , 0) ; std :: future :: poll_fn (| cx | service . poll_ready (cx)) . await . unwrap () ; assert_eq ! (counter . get () , 0) ; let response_future = service . call (Request :: new (Body :: empty ())) ; assert_eq ! (counter . get () , 1) ; let response = response_future . await . unwrap () ; assert_eq ! (counter . get () , 1) ; let body = response . into_body () ; crate :: test_helpers :: to_bytes (body) . await . unwrap () ; assert_eq ! (counter . get () , 0) ; } async fn echo (req : Request < Body >) -> Result < Response < Body > , BoxError > { Ok (Response :: new (req . into_body ())) } }
};
}
