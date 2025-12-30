// Generated macro for put (function)
macro_rules! Depcrate_filters_methodput {
() => {
// Module: crate::filters::method
// Provides: {"put"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `PUT`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let put_only = warp::put().map(warp::reply);"] # [doc = " ```"] pub fn put () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: PUT) }
};
}
