// Generated macro for impl_753 (impl)
macro_rules! Depcrate_replyimpl_753 {
() => {
// Module: crate::reply
// Provides: {"impl_753"}
// Dependencies: {}
impl < T > Reply for Html < T > where crate :: bodyt :: Body : From < T > , T : Send , { # [inline] fn into_response (self) -> Response { let mut res = Response :: new (Body :: from (self . body)) ; res . headers_mut () . insert (CONTENT_TYPE , HeaderValue :: from_static ("text/html; charset=utf-8") ,) ; res } }
};
}
