// Generated macro for preflight_request_headers (function)
macro_rules! Depcrate_corspreflight_request_headers {
() => {
// Module: crate::cors
// Provides: {"preflight_request_headers"}
// Dependencies: {}
# [doc = " Returns an iterator over the three request headers that may be involved in a CORS preflight request."] # [doc = ""] # [doc = " This is the default set of header names returned in the `vary` header"] pub fn preflight_request_headers () -> impl Iterator < Item = HeaderName > { # [allow (deprecated)] array :: IntoIter :: new ([header :: ORIGIN , header :: ACCESS_CONTROL_REQUEST_METHOD , header :: ACCESS_CONTROL_REQUEST_HEADERS ,]) }
};
}
