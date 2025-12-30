// Generated macro for header (function)
macro_rules! Depcrate_filters_replyheader {
() => {
// Module: crate::filters::reply
// Provides: {"header"}
// Dependencies: {}
# [doc = " Wrap a [`Filter`] that adds a header to the reply."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This **only** adds a header if the underlying filter is successful, and"] # [doc = " returns a [`Reply`] If the underlying filter was rejected, the"] # [doc = " header is not added."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Always set `foo: bar` header."] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::reply::with::header(\"foo\", \"bar\"));"] # [doc = " ```"] pub fn header < K , V > (name : K , value : V) -> WithHeader where HeaderName : TryFrom < K > , < HeaderName as TryFrom < K > > :: Error : Into < http :: Error > , HeaderValue : TryFrom < V > , < HeaderValue as TryFrom < V > > :: Error : Into < http :: Error > , { let (name , value) = assert_name_and_value (name , value) ; WithHeader { name , value } }
};
}
