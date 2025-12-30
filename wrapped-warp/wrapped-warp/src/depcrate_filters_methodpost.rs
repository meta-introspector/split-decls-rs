// Generated macro for post (function)
macro_rules! Depcrate_filters_methodpost {
() => {
// Module: crate::filters::method
// Provides: {"post"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `POST`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let post_only = warp::post().map(warp::reply);"] # [doc = " ```"] pub fn post () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: POST) }
};
}
