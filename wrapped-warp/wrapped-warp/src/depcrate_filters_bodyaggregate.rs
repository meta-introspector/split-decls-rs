// Generated macro for aggregate (function)
macro_rules! Depcrate_filters_bodyaggregate {
() => {
// Module: crate::filters::body
// Provides: {"aggregate"}
// Dependencies: {}
# [doc = " Returns a `Filter` that matches any request and extracts a `Future` of an"] # [doc = " aggregated body."] # [doc = ""] # [doc = " The `Buf` may contain multiple, non-contiguous buffers. This can be more"] # [doc = " performant (by reducing copies) when receiving large bodies."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This does not have a default size limit, it would be wise to use one to"] # [doc = " prevent a overly large request from using too much memory."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Buf, Filter};"] # [doc = ""] # [doc = " fn full_body(mut body: impl Buf) {"] # [doc = "     // It could have several non-contiguous slices of memory..."] # [doc = "     while body.has_remaining() {"] # [doc = "         println!(\"slice = {:?}\", body.chunk());"] # [doc = "         let cnt = body.chunk().len();"] # [doc = "         body.advance(cnt);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let route = warp::body::content_length_limit(1024 * 32)"] # [doc = "     .and(warp::body::aggregate())"] # [doc = "     .map(full_body);"] # [doc = " ```"] pub fn aggregate () -> impl Filter < Extract = (impl Buf ,) , Error = Rejection > + Copy { body () . and_then (| mut body : crate :: bodyt :: Body | async move { http_body_util :: BodyExt :: collect (& mut body) . await . map (| collected | collected . aggregate ()) . map_err (| err | { tracing :: debug ! ("aggregate error: {}" , err) ; reject :: known (BodyReadError (err)) }) }) }
};
}
