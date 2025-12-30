// Generated macro for param (function)
macro_rules! Depcrate_filters_pathparam {
() => {
// Module: crate::filters::path
// Provides: {"param"}
// Dependencies: {}
# [doc = " Extract a parameter from a path segment."] # [doc = ""] # [doc = " This will try to parse a value from the current request path"] # [doc = " segment, and if successful, the value is returned as the `Filter`'s"] # [doc = " \"extracted\" value."] # [doc = ""] # [doc = " If the value could not be parsed, rejects with a `404 Not Found`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::path::param()"] # [doc = "     .map(|id: u32| {"] # [doc = "         format!(\"You asked for /{}\", id)"] # [doc = "     });"] # [doc = " ```"] pub fn param < T : FromStr + Send + 'static > () -> impl Filter < Extract = One < T > , Error = Rejection > + Copy { filter_segment (| seg | { tracing :: trace ! ("param?: {:?}" , seg) ; if seg . is_empty () { return Err (reject :: not_found ()) ; } T :: from_str (seg) . map (one) . map_err (| _ | reject :: not_found ()) }) }
};
}
