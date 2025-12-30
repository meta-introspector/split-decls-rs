// Generated macro for end (function)
macro_rules! Depcrate_filters_pathend {
() => {
// Module: crate::filters::path
// Provides: {"end"}
// Dependencies: {}
# [doc = " Matches the end of a route."] # [doc = ""] # [doc = " Note that _not_ including `end()` may result in shorter paths like"] # [doc = " `/math` unintentionally matching `/math/sum`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Matches '/'"] # [doc = " let hello = warp::path::end()"] # [doc = "     .map(|| \"Hello, World!\");"] # [doc = " ```"] pub fn end () -> impl Filter < Extract = () , Error = Rejection > + Copy { filter_fn (move | route | { if route . path () . is_empty () { future :: ok (()) } else { future :: err (reject :: not_found ()) } }) }
};
}
