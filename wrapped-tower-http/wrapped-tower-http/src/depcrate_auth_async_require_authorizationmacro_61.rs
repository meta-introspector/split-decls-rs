// Generated macro for macro_61 (macro)
macro_rules! Depcrate_auth_async_require_authorizationmacro_61 {
() => {
// Module: crate::auth::async_require_authorization
// Provides: {"macro_61"}
// Dependencies: {}
pin_project ! { # [doc = " Response future for [`AsyncRequireAuthorization`]."] pub struct ResponseFuture < Auth , S , ReqBody > where Auth : AsyncAuthorizeRequest < ReqBody >, S : Service < Request < Auth :: RequestBody >>, { # [pin] state : State < Auth :: Future , S :: Future >, service : S , } }
};
}
