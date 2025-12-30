// Generated macro for impl_505 (impl)
macro_rules! Depcrate_trace_make_spanimpl_505 {
() => {
// Module: crate::trace::make_span
// Provides: {"impl_505"}
// Dependencies: {}
impl DefaultMakeSpan { # [doc = " Create a new `DefaultMakeSpan`."] pub fn new () -> Self { Self { level : DEFAULT_MESSAGE_LEVEL , include_headers : false , } } # [doc = " Set the [`Level`] used for the [tracing span]."] # [doc = ""] # [doc = " Defaults to [`Level::DEBUG`]."] # [doc = ""] # [doc = " [tracing span]: https://docs.rs/tracing/latest/tracing/#spans"] pub fn level (mut self , level : Level) -> Self { self . level = level ; self } # [doc = " Include request headers on the [`Span`]."] # [doc = ""] # [doc = " By default headers are not included."] # [doc = ""] # [doc = " [`Span`]: tracing::Span"] pub fn include_headers (mut self , include_headers : bool) -> Self { self . include_headers = include_headers ; self } }
};
}
