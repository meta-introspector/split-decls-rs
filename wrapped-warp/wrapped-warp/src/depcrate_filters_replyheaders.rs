// Generated macro for headers (function)
macro_rules! Depcrate_filters_replyheaders {
() => {
// Module: crate::filters::reply
// Provides: {"headers"}
// Dependencies: {}
# [doc = " Wrap a [`Filter`] that adds multiple headers to the reply."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This **only** adds a header if the underlying filter is successful, and"] # [doc = " returns a [`Reply`] If the underlying filter was rejected, the"] # [doc = " header is not added."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::http::header::{HeaderMap, HeaderValue};"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let mut headers = HeaderMap::new();"] # [doc = " headers.insert(\"server\", HeaderValue::from_static(\"wee/0\"));"] # [doc = " headers.insert(\"foo\", HeaderValue::from_static(\"bar\"));"] # [doc = ""] # [doc = " // Always set `server: wee/0` and `foo: bar` headers."] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::reply::with::headers(headers));"] # [doc = " ```"] pub fn headers (headers : HeaderMap) -> WithHeaders { WithHeaders { headers : Arc :: new (headers) , } }
};
}
