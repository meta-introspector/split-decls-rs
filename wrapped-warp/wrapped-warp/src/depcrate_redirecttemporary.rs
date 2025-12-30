// Generated macro for temporary (function)
macro_rules! Depcrate_redirecttemporary {
() => {
// Module: crate::redirect
// Provides: {"temporary"}
// Dependencies: {}
# [doc = " HTTP 307 Temporary Redirect:"] # [doc = " Description: The requested resource can be found at a different URL temporarily."] # [doc = " Usage: Similar to 302, but explicitly defined as a temporary redirect. The main difference between 307 and 302 is that 307 preserves the method of the original request when redirecting. If the original request was a POST, the subsequent request to the new URL will also be a POST."] # [doc = " Common Use Case: Temporary redirects that should preserve the original request method."] # [doc = ""] # [doc = " This is similar to [`see_other`](fn@see_other) but the HTTP method and the body of the request"] # [doc = " to the new location will be the same as the method and body of the current request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{http::Uri, Filter};"] # [doc = ""] # [doc = " let route = warp::path(\"v1\")"] # [doc = "     .map(|| {"] # [doc = "         warp::redirect::temporary(Uri::from_static(\"/v2\"))"] # [doc = "     });"] # [doc = " ```"] pub fn temporary (uri : impl AsLocation) -> impl Reply { reply :: with_header (StatusCode :: TEMPORARY_REDIRECT , header :: LOCATION , uri . header_value () ,) }
};
}
