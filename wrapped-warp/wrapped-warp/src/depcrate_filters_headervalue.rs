// Generated macro for value (function)
macro_rules! Depcrate_filters_headervalue {
() => {
// Module: crate::filters::header
// Provides: {"value"}
// Dependencies: {}
# [doc = " Create a `Filter` that gets a `HeaderValue` for the name."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Filter, http::header::HeaderValue};"] # [doc = ""] # [doc = " let filter = warp::header::value(\"x-token\")"] # [doc = "     .map(|value: HeaderValue| {"] # [doc = "         format!(\"header value bytes: {:?}\", value)"] # [doc = "     });"] # [doc = " ```"] pub fn value (name : & 'static str ,) -> impl Filter < Extract = One < HeaderValue > , Error = Rejection > + Copy { filter_fn_one (move | route | { tracing :: trace ! ("value({:?})" , name) ; let route = route . headers () . get (name) . cloned () . ok_or_else (| | reject :: missing_header (name)) ; future :: ready (route) }) }
};
}
