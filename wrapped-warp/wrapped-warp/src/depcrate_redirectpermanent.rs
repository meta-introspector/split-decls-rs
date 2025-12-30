// Generated macro for permanent (function)
macro_rules! Depcrate_redirectpermanent {
() => {
// Module: crate::redirect
// Provides: {"permanent"}
// Dependencies: {}
# [doc = " HTTP 308 Permanent Redirect"] # [doc = " Description: The requested resource has been permanently moved to a new URL, and future requests should use the new URL."] # [doc = " Usage: Similar to 301, but like 307, it preserves the original request method when redirecting. It indicates that the redirection is permanent, and browsers and clients will cache this redirect like they do for 301."] # [doc = ""] # [doc = " This is similar to [`redirect`](fn@redirect) but the HTTP method of the request to the new"] # [doc = " location will be the same as the method of the current request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{http::Uri, Filter};"] # [doc = ""] # [doc = " let route = warp::path(\"v1\")"] # [doc = "     .map(|| {"] # [doc = "         warp::redirect::permanent(Uri::from_static(\"/v2\"))"] # [doc = "     });"] # [doc = " ```"] pub fn permanent (uri : impl AsLocation) -> impl Reply { reply :: with_header (StatusCode :: PERMANENT_REDIRECT , header :: LOCATION , uri . header_value () ,) }
};
}
