// Generated macro for with_header (function)
macro_rules! Depcrate_replywith_header {
() => {
// Module: crate::reply
// Provides: {"with_header"}
// Dependencies: {}
# [doc = " Wrap an `impl Reply` to add a header when rendering."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .map(|reply| {"] # [doc = "         warp::reply::with_header(reply, \"server\", \"warp\")"] # [doc = "     });"] # [doc = " ```"] pub fn with_header < T : Reply , K , V > (reply : T , name : K , value : V) -> WithHeader < T > where HeaderName : TryFrom < K > , < HeaderName as TryFrom < K > > :: Error : Into < http :: Error > , HeaderValue : TryFrom < V > , < HeaderValue as TryFrom < V > > :: Error : Into < http :: Error > , { let header = match < HeaderName as TryFrom < K > > :: try_from (name) { Ok (name) => match < HeaderValue as TryFrom < V > > :: try_from (value) { Ok (value) => Some ((name , value)) , Err (err) => { let err = err . into () ; tracing :: error ! ("with_header value error: {}" , err) ; None } } , Err (err) => { let err = err . into () ; tracing :: error ! ("with_header name error: {}" , err) ; None } } ; WithHeader { header , reply } }
};
}
