// Generated macro for delete (function)
macro_rules! Depcrate_filters_methoddelete {
() => {
// Module: crate::filters::method
// Provides: {"delete"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `DELETE`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let delete_only = warp::delete().map(warp::reply);"] # [doc = " ```"] pub fn delete () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: DELETE) }
};
}
