// Generated macro for exact (function)
macro_rules! Depcrate_filters_headerexact {
() => {
// Module: crate::filters::header
// Provides: {"exact"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires a header to match the value exactly."] # [doc = ""] # [doc = " This `Filter` will look for a header with supplied name and the exact"] # [doc = " value, otherwise rejects the request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // Require `dnt: 1` header to be set."] # [doc = " let must_dnt = warp::header::exact(\"dnt\", \"1\");"] # [doc = " ```"] pub fn exact (name : & 'static str , value : & 'static str ,) -> impl Filter < Extract = () , Error = Rejection > + Copy { filter_fn (move | route | { tracing :: trace ! ("exact?({:?}, {:?})" , name , value) ; let route = route . headers () . get (name) . ok_or_else (| | reject :: missing_header (name)) . and_then (| val | { if val == value { Ok (()) } else { Err (reject :: invalid_header (name)) } }) ; future :: ready (route) }) }
};
}
