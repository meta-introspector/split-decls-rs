// Generated macro for create_error_response (function)
macro_rules! Depcrate_limit_bodycreate_error_response {
() => {
// Module: crate::limit::body
// Provides: {"create_error_response"}
// Dependencies: {}
pub (crate) fn create_error_response < B > () -> Response < ResponseBody < B > > where B : Body , { let mut res = Response :: new (ResponseBody :: payload_too_large ()) ; * res . status_mut () = StatusCode :: PAYLOAD_TOO_LARGE ; # [allow (clippy :: declare_interior_mutable_const)] const TEXT_PLAIN : HeaderValue = HeaderValue :: from_static ("text/plain; charset=utf-8") ; res . headers_mut () . insert (http :: header :: CONTENT_TYPE , TEXT_PLAIN) ; res }
};
}
