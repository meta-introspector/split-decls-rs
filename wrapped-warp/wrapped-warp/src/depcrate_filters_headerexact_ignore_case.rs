// Generated macro for exact_ignore_case (function)
macro_rules! Depcrate_filters_headerexact_ignore_case {
() => {
// Module: crate::filters::header
// Provides: {"exact_ignore_case"}
// Dependencies: {}
# [doc = " Create a `Filter` that requires a header to match the value exactly."] # [doc = ""] # [doc = " This `Filter` will look for a header with supplied name and the exact"] # [doc = " value, ignoring ASCII case, otherwise rejects the request."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " // Require `connection: keep-alive` header to be set."] # [doc = " let keep_alive = warp::header::exact_ignore_case(\"connection\", \"keep-alive\");"] # [doc = " ```"] pub fn exact_ignore_case (name : & 'static str , value : & 'static str ,) -> impl Filter < Extract = () , Error = Rejection > + Copy { filter_fn (move | route | { tracing :: trace ! ("exact_ignore_case({:?}, {:?})" , name , value) ; let route = route . headers () . get (name) . ok_or_else (| | reject :: missing_header (name)) . and_then (| val | { if val . as_bytes () . eq_ignore_ascii_case (value . as_bytes ()) { Ok (()) } else { Err (reject :: invalid_header (name)) } }) ; future :: ready (route) }) }
};
}
