// Generated macro for single_endpoint (function)
macro_rules! Depcrate_balance_p2c_testsingle_endpoint {
() => {
// Module: crate::balance::p2c::test
// Provides: {"single_endpoint"}
// Dependencies: {}
# [tokio :: test] async fn single_endpoint () { let (mut svc , mut handle) = mock :: spawn_with (| s | { let mock = load :: Constant :: new (s , 0) ; let disco = ServiceList :: new (vec ! [mock] . into_iter ()) ; Balance :: new (disco) }) ; handle . allow (0) ; assert_pending ! (svc . poll_ready ()) ; assert_eq ! (svc . get_ref () . len () , 1 , "balancer must have discovered endpoint") ; handle . allow (1) ; assert_ready_ok ! (svc . poll_ready ()) ; let mut fut = task :: spawn (svc . call (())) ; assert_request_eq ! (handle , ()) . send_response (1) ; assert_eq ! (assert_ready_ok ! (fut . poll ()) , 1) ; handle . allow (1) ; assert_ready_ok ! (svc . poll_ready ()) ; handle . send_error ("endpoint lost") ; assert_pending ! (svc . poll_ready ()) ; assert ! (svc . get_ref () . is_empty () , "balancer must drop failed endpoints") ; }
};
}
