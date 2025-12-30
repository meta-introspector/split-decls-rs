// Generated macro for redirect (function)
macro_rules! Depcrate_redirectredirect {
() => {
// Module: crate::redirect
// Provides: {"redirect"}
// Dependencies: {}
# [doc = " HTTP 301 Moved Permanently"] # [doc = " Description: The requested resource has been permanently moved to a new URL."] # [doc = " Usage: It is used when a URL has permanently moved to a new location. Search engines will update their index to the new URL. Browsers and clients will automatically cache this redirect, so subsequent requests for the old URL will automatically go to the new URL without making a request to the old URL."] # [doc = " Common Use Case: Changing domain names, restructuring website URLs."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{http::Uri, Filter};"] # [doc = ""] # [doc = " let route = warp::path(\"v1\")"] # [doc = "     .map(|| {"] # [doc = "         warp::redirect(Uri::from_static(\"/v2\"))"] # [doc = "     });"] # [doc = " ```"] pub fn redirect (uri : impl AsLocation) -> impl Reply { reply :: with_header (StatusCode :: MOVED_PERMANENTLY , header :: LOCATION , uri . header_value () ,) }
};
}
