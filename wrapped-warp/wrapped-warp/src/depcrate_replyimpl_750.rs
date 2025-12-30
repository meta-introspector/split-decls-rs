// Generated macro for impl_750 (impl)
macro_rules! Depcrate_replyimpl_750 {
() => {
// Module: crate::reply
// Provides: {"impl_750"}
// Dependencies: {}
impl Reply for Json { # [inline] fn into_response (self) -> Response { match self . inner { Ok (body) => { let mut res = Response :: new (body . into ()) ; res . headers_mut () . insert (CONTENT_TYPE , HeaderValue :: from_static ("application/json")) ; res } Err (()) => StatusCode :: INTERNAL_SERVER_ERROR . into_response () , } } }
};
}
