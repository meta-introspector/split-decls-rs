// Generated macro for impl_769 (impl)
macro_rules! Depcrate_replyimpl_769 {
() => {
// Module: crate::reply
// Provides: {"impl_769"}
// Dependencies: {}
impl Reply for Vec < u8 > { # [inline] fn into_response (self) -> Response { :: http :: Response :: builder () . header (CONTENT_TYPE , HeaderValue :: from_static ("application/octet-stream") ,) . body (Body :: from (self)) . unwrap () } }
};
}
