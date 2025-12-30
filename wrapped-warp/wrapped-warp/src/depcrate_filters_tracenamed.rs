// Generated macro for named (function)
macro_rules! Depcrate_filters_tracenamed {
() => {
// Module: crate::filters::trace
// Provides: {"named"}
// Dependencies: {}
# [doc = " Create a wrapping filter that instruments every request with a `tracing`"] # [doc = " [`Span`] at the [`DEBUG`] level representing a named context."] # [doc = ""] # [doc = " This can be used to instrument multiple routes with their own sub-spans in a"] # [doc = " per-request trace."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::Filter;"] # [doc = ""] # [doc = " let hello = warp::path(\"hello\")"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::trace::named(\"hello\"));"] # [doc = ""] # [doc = " let goodbye = warp::path(\"goodbye\")"] # [doc = "     .map(warp::reply)"] # [doc = "     .with(warp::trace::named(\"goodbye\"));"] # [doc = ""] # [doc = " let routes = hello.or(goodbye);"] # [doc = " ```"] # [doc = ""] # [doc = " [`Span`]: https://docs.rs/tracing/latest/tracing/#spans"] # [doc = " [`DEBUG`]: https://docs.rs/tracing/0.1.16/tracing/struct.Level.html#associatedconstant.DEBUG"] pub fn named (name : & 'static str) -> Trace < impl Fn (Info < '_ >) -> Span + Copy > { trace (move | _ | tracing :: debug_span ! ("context" , "{}" , name ,)) }
};
}
