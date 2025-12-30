// Generated macro for peek (function)
macro_rules! Depcrate_filters_pathpeek {
() => {
// Module: crate::filters::path
// Provides: {"peek"}
// Dependencies: {}
# [doc = " Peek at the unmatched tail of the path, without affecting the matched path."] # [doc = ""] # [doc = " This will return a `Peek`, which allows access to the rest of the path"] # [doc = " that previous filters have not already matched. This differs from `tail`"] # [doc = " in that `peek` will **not** set the entire path as matched."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::path(\"foo\")"] # [doc = "     .and(warp::path::peek())"] # [doc = "     .map(|peek| {"] # [doc = "         // GET /foo/bar/baz would return \"bar/baz\"."] # [doc = "         format!(\"The path after foo is {:?}\", peek)"] # [doc = "     });"] # [doc = " ```"] pub fn peek () -> impl Filter < Extract = One < Peek > , Error = Infallible > + Copy { filter_fn (move | route | { let path = path_and_query (route) ; let idx = route . matched_path_index () ; future :: ok (one (Peek { path , start_index : idx , })) }) }
};
}
