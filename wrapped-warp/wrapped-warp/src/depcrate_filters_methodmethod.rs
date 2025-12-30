// Generated macro for method (function)
macro_rules! Depcrate_filters_methodmethod {
() => {
// Module: crate::filters::method
// Provides: {"method"}
// Dependencies: {}
# [doc = " Extract the `Method` from the request."] # [doc = ""] # [doc = " This never rejects a request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::method()"] # [doc = "     .map(|method| {"] # [doc = "         format!(\"You sent a {} request!\", method)"] # [doc = "     });"] # [doc = " ```"] pub fn method () -> impl Filter < Extract = One < Method > , Error = Infallible > + Copy { filter_fn_one (| route | future :: ok :: < _ , Infallible > (route . method () . clone ())) }
};
}
