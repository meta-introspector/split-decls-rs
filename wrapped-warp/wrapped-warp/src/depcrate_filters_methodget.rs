// Generated macro for get (function)
macro_rules! Depcrate_filters_methodget {
() => {
// Module: crate::filters::method
// Provides: {"get"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `GET`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let get_only = warp::get().map(warp::reply);"] # [doc = " ```"] pub fn get () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: GET) }
};
}
