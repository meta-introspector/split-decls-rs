// Generated macro for bytes (function)
macro_rules! Depcrate_filters_bodybytes {
() => {
// Module: crate::filters::body
// Provides: {"bytes"}
// Dependencies: {}
# [doc = " Returns a `Filter` that matches any request and extracts a `Future` of a"] # [doc = " concatenated body."] # [doc = ""] # [doc = " The contents of the body will be flattened into a single contiguous"] # [doc = " `Bytes`, which may require memory copies. If you don't require a"] # [doc = " contiguous buffer, using `aggregate` can be give better performance."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This does not have a default size limit, it would be wise to use one to"] # [doc = " prevent a overly large request from using too much memory."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Buf, Filter};"] # [doc = ""] # [doc = " let route = warp::body::content_length_limit(1024 * 32)"] # [doc = "     .and(warp::body::bytes())"] # [doc = "     .map(|bytes: bytes::Bytes| {"] # [doc = "         println!(\"bytes = {:?}\", bytes);"] # [doc = "     });"] # [doc = " ```"] pub fn bytes () -> impl Filter < Extract = (Bytes ,) , Error = Rejection > + Copy { body () . and_then (| mut body | async move { BodyExt :: collect (& mut body) . await . map (| b | b . to_bytes ()) . map_err (| err | { tracing :: debug ! ("to_bytes error: {}" , err) ; reject :: known (BodyReadError (err)) }) }) }
};
}
