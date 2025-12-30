// Generated macro for head (function)
macro_rules! Depcrate_filters_methodhead {
() => {
// Module: crate::filters::method
// Provides: {"head"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `HEAD`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let head_only = warp::head().map(warp::reply);"] # [doc = " ```"] pub fn head () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: HEAD) }
};
}
