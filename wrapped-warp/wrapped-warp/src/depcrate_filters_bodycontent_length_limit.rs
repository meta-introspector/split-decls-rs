// Generated macro for content_length_limit (function)
macro_rules! Depcrate_filters_bodycontent_length_limit {
() => {
// Module: crate::filters::body
// Provides: {"content_length_limit"}
// Dependencies: {}
# [doc = " Require a `content-length` header to have a value no greater than some limit."] # [doc = ""] # [doc = " Rejects if `content-length` header is missing, is invalid, or has a number"] # [doc = " larger than the limit provided."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " // Limit the upload to 4kb..."] # [doc = " let upload = warp::body::content_length_limit(4096)"] # [doc = "     .and(warp::body::aggregate());"] # [doc = " ```"] pub fn content_length_limit (limit : u64) -> impl Filter < Extract = () , Error = Rejection > + Copy { crate :: filters :: header :: header2 () . map_err (crate :: filter :: Internal , | _ | { tracing :: debug ! ("content-length missing") ; reject :: length_required () }) . and_then (move | ContentLength (length) | { if length <= limit { future :: ok (()) } else { tracing :: debug ! ("content-length: {} is over limit {}" , length , limit) ; future :: err (reject :: payload_too_large ()) } }) . untuple_one () }
};
}
