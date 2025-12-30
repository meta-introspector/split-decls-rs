// Generated macro for tail (function)
macro_rules! Depcrate_filters_pathtail {
() => {
// Module: crate::filters::path
// Provides: {"tail"}
// Dependencies: {}
# [doc = " Extract the unmatched tail of the path."] # [doc = ""] # [doc = " This will return a `Tail`, which allows access to the rest of the path"] # [doc = " that previous filters have not already matched."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::path(\"foo\")"] # [doc = "     .and(warp::path::tail())"] # [doc = "     .map(|tail| {"] # [doc = "         // GET /foo/bar/baz would return \"bar/baz\"."] # [doc = "         format!(\"The tail after foo is {:?}\", tail)"] # [doc = "     });"] # [doc = " ```"] pub fn tail () -> impl Filter < Extract = One < Tail > , Error = Infallible > + Copy { filter_fn (move | route | { let path = path_and_query (route) ; let idx = route . matched_path_index () ; let end = path . path () . len () - idx ; route . set_unmatched_path (end) ; future :: ok (one (Tail { path , start_index : idx , })) }) }
};
}
