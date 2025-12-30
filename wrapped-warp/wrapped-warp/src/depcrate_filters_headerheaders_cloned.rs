// Generated macro for headers_cloned (function)
macro_rules! Depcrate_filters_headerheaders_cloned {
() => {
// Module: crate::filters::header
// Provides: {"headers_cloned"}
// Dependencies: {}
# [doc = " Create a `Filter` that returns a clone of the request's `HeaderMap`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Filter, http::HeaderMap};"] # [doc = ""] # [doc = " let headers = warp::header::headers_cloned()"] # [doc = "     .map(|headers: HeaderMap| {"] # [doc = "         format!(\"header count: {}\", headers.len())"] # [doc = "     });"] # [doc = " ```"] pub fn headers_cloned () -> impl Filter < Extract = One < HeaderMap > , Error = Infallible > + Copy { filter_fn_one (| route | future :: ok (route . headers () . clone ())) }
};
}
