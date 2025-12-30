// Generated macro for WithHeader (struct)
macro_rules! Depcrate_replyWithHeader {
() => {
// Module: crate::reply
// Provides: {"WithHeader"}
// Dependencies: {}
# [doc = " Wraps an `impl Reply` and adds a header when rendering."] # [doc = ""] # [doc = " Returned by `warp::reply::with_header`."] # [derive (Debug)] pub struct WithHeader < T > { header : Option < (HeaderName , HeaderValue) > , reply : T , }
};
}
