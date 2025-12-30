// Generated macro for default_header (function)
macro_rules! Depcrate_filters_replydefault_header {
() => {
// Module: crate::filters::reply
// Provides: {"default_header"}
// Dependencies: {}
# [doc = " Wrap a [`Filter`] that adds a header to the reply, if they"] # [doc = " aren't already set."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This **only** adds a header if the underlying filter is successful, and"] # [doc = " returns a [`Reply`] If the underlying filter was rejected, the"] # [doc = " header is not added."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Set `server: warp` if not already set."] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::reply::with::default_header(\"server\", \"warp\"));"] # [doc = " ```"] pub fn default_header < K , V > (name : K , value : V) -> WithDefaultHeader where HeaderName : TryFrom < K > , < HeaderName as TryFrom < K > > :: Error : Into < http :: Error > , HeaderValue : TryFrom < V > , < HeaderValue as TryFrom < V > > :: Error : Into < http :: Error > , { let (name , value) = assert_name_and_value (name , value) ; WithDefaultHeader { name , value } }
};
}
