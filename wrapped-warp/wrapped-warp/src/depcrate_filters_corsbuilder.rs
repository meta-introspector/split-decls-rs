// Generated macro for Builder (struct)
macro_rules! Depcrate_filters_corsBuilder {
() => {
// Module: crate::filters::cors
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A constructed via `warp::cors()`."] # [derive (Clone , Debug)] pub struct Builder { credentials : bool , allowed_headers : HashSet < HeaderName > , exposed_headers : HashSet < HeaderName > , max_age : Option < u64 > , methods : HashSet < http :: Method > , origins : Option < HashSet < HeaderValue > > , }
};
}
