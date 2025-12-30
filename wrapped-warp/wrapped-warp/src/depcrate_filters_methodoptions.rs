// Generated macro for options (function)
macro_rules! Depcrate_filters_methodoptions {
() => {
// Module: crate::filters::method
// Provides: {"options"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `OPTIONS`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let options_only = warp::options().map(warp::reply);"] # [doc = " ```"] pub fn options () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: OPTIONS) }
};
}
