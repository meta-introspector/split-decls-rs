// Generated macro for patch (function)
macro_rules! Depcrate_filters_methodpatch {
() => {
// Module: crate::filters::method
// Provides: {"patch"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires the request method to be `PATCH`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let patch_only = warp::patch().map(warp::reply);"] # [doc = " ```"] pub fn patch () -> impl Filter < Extract = () , Error = Rejection > + Copy { method_is (| | & Method :: PATCH) }
};
}
