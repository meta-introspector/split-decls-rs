// Generated macro for found (function)
macro_rules! Depcrate_redirectfound {
() => {
// Module: crate::redirect
// Provides: {"found"}
// Dependencies: {}
# [doc = " HTTP 302 Found (or Temporary Redirect)"] # [doc = " Description: The requested resource can be found at a different URL temporarily."] # [doc = " Usage: Historically, this status code was used for temporary redirects. However, its meaning was often misunderstood, and different clients treated it differently. As a result, it is recommended to use 307 (or 303) for temporary redirects instead."] # [doc = " Common Use Case: Rarely used directly due to ambiguity; replaced by 307 or 303."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{http::Uri, Filter};"] # [doc = ""] # [doc = " let route = warp::path(\"v1\")"] # [doc = "     .map(|| {"] # [doc = "         warp::redirect::found(Uri::from_static(\"/v2\"))"] # [doc = "     });"] # [doc = " ```"] pub fn found (uri : impl AsLocation) -> impl Reply { reply :: with_header (StatusCode :: FOUND , header :: LOCATION , uri . header_value ()) }
};
}
