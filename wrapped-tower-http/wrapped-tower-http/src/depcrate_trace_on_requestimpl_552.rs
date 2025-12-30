// Generated macro for impl_552 (impl)
macro_rules! Depcrate_trace_on_requestimpl_552 {
() => {
// Module: crate::trace::on_request
// Provides: {"impl_552"}
// Dependencies: {}
impl DefaultOnRequest { # [doc = " Create a new `DefaultOnRequest`."] pub fn new () -> Self { Self :: default () } # [doc = " Set the [`Level`] used for [tracing events]."] # [doc = ""] # [doc = " Please note that while this will set the level for the tracing events"] # [doc = " themselves, it might cause them to lack expected information, like"] # [doc = " request method or path. You can address this using"] # [doc = " [`DefaultMakeSpan::level`]."] # [doc = ""] # [doc = " Defaults to [`Level::DEBUG`]."] # [doc = ""] # [doc = " [tracing events]: https://docs.rs/tracing/latest/tracing/#events"] # [doc = " [`DefaultMakeSpan::level`]: crate::trace::DefaultMakeSpan::level"] pub fn level (mut self , level : Level) -> Self { self . level = level ; self } }
};
}
