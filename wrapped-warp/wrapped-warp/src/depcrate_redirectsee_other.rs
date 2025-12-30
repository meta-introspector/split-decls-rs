// Generated macro for see_other (function)
macro_rules! Depcrate_redirectsee_other {
() => {
// Module: crate::redirect
// Provides: {"see_other"}
// Dependencies: {}
# [doc = " HTTP 303 See Other"] # [doc = " Description: The response to the request can be found at a different URL, and the client should retrieve it using the GET method."] # [doc = " Usage: It is typically used to redirect the client to another URL using a GET request after processing a POST request. It ensures that the client doesn't repeat the POST request if they refresh the page."] # [doc = " Common Use Case: After form submissions or any non-idempotent request."] # [doc = ""] # [doc = " The HTTP method of the request to the new location will always be `GET`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{http::Uri, Filter};"] # [doc = ""] # [doc = " let route = warp::path(\"v1\")"] # [doc = "     .map(|| {"] # [doc = "         warp::redirect::see_other(Uri::from_static(\"/v2\"))"] # [doc = "     });"] # [doc = " ```"] pub fn see_other (uri : impl AsLocation) -> impl Reply { reply :: with_header (StatusCode :: SEE_OTHER , header :: LOCATION , uri . header_value ()) }
};
}
