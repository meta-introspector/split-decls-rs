// Generated macro for impl_772 (impl)
macro_rules! Depcrate_replyimpl_772 {
() => {
// Module: crate::reply
// Provides: {"impl_772"}
// Dependencies: {}
impl Reply for & 'static [u8] { # [inline] fn into_response (self) -> Response { :: http :: Response :: builder () . header (CONTENT_TYPE , HeaderValue :: from_static ("application/octet-stream") ,) . body (Body :: from (bytes :: Bytes :: from_static (self))) . unwrap () } }
};
}
