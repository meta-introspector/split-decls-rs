// Generated macro for trace (function)
macro_rules! Depcrate_filters_tracetrace {
() => {
// Module: crate::filters::trace
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Create a wrapping filter that instruments every request with a custom"] # [doc = " `tracing` [`Span`] provided by a function."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let route = warp::any()"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::trace(|info| {"] # [doc = "         // Create a span using tracing macros"] # [doc = "         tracing::info_span!("] # [doc = "             \"request\","] # [doc = "             method = %info.method(),"] # [doc = "             path = %info.path(),"] # [doc = "         )"] # [doc = "     }));"] # [doc = " ```"] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/#spans"] pub fn trace < F > (func : F) -> Trace < F > where F : Fn (Info < '_ >) -> Span + Clone , { Trace { func } }
};
}
