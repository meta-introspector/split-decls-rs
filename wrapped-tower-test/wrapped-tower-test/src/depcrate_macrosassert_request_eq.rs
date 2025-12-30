// Generated macro for assert_request_eq (macro)
macro_rules! Depcrate_macrosassert_request_eq {
() => {
// Module: crate::macros
// Provides: {"assert_request_eq"}
// Dependencies: {}
# [doc = " Asserts that the mock handle receives a new request equal to the given"] # [doc = " value."] # [doc = ""] # [doc = " On success, the [`SendResponse`] handle for the matched request is returned,"] # [doc = " allowing the caller to respond to the request. On failure, the macro panics."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_service::Service;"] # [doc = " use tower_test::{mock, assert_request_eq};"] # [doc = " use tokio_test::assert_ready;"] # [doc = ""] # [doc = " # async fn test() {"] # [doc = " let (mut service, mut handle) = mock::spawn();"] # [doc = ""] # [doc = " assert_ready!(service.poll_ready());"] # [doc = ""] # [doc = " let response = service.call(\"hello\");"] # [doc = ""] # [doc = " assert_request_eq!(handle, \"hello\").send_response(\"world\");"] # [doc = ""] # [doc = " assert_eq!(response.await.unwrap(), \"world\");"] # [doc = " # }"] # [doc = " ```"] # [doc = " [`SendResponse`]: crate::mock::SendResponse"] # [macro_export] macro_rules ! assert_request_eq { ($ mock_handle : expr , $ expect : expr) => { assert_request_eq ! ($ mock_handle , $ expect ,) } ; ($ mock_handle : expr , $ expect : expr , $ ($ arg : tt) *) => { { let (actual , send_response) = match $ mock_handle . next_request () . await { Some (r) => r , None => panic ! ("expected a request but none was received.") , } ; # [allow (clippy :: bool_assert_comparison)] { assert_eq ! (actual , $ expect , $ ($ arg) *) ; } send_response } } ; }
};
}
