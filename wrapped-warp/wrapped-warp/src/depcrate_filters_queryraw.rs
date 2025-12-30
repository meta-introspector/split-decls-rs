// Generated macro for raw (function)
macro_rules! Depcrate_filters_queryraw {
() => {
// Module: crate::filters::query
// Provides: {"raw"}
// Dependencies: {}
# [doc = " Creates a `Filter` that returns the raw query string as type String."] pub fn raw () -> impl Filter < Extract = One < String > , Error = Rejection > + Copy { filter_fn_one (| route | { let route = route . query () . map (| q | q . to_owned ()) . map (Ok) . unwrap_or_else (| | Err (reject :: invalid_query ())) ; future :: ready (route) }) }
};
}
