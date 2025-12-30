// Generated macro for ErrorLayer (struct)
macro_rules! Depcrate_layerErrorLayer {
() => {
// Module: crate::layer
// Provides: {"ErrorLayer"}
// Dependencies: {}
# [doc = " A subscriber [`Layer`] that enables capturing [`SpanTrace`]s."] # [doc = ""] # [doc = " Optionally, this type may be constructed with a [field formatter] to use"] # [doc = " when formatting the fields of each span in a trace. When no formatter is"] # [doc = " provided, the [default format] is used instead."] # [doc = ""] # [doc = " [`Layer`]: tracing_subscriber::layer::Layer"] # [doc = " [`SpanTrace`]: super::SpanTrace"] # [doc = " [field formatter]: tracing_subscriber::fmt::FormatFields"] # [doc = " [default format]: tracing_subscriber::fmt::format::DefaultFields"] pub struct ErrorLayer < S , F = DefaultFields > { format : F , get_context : WithContext , _subscriber : PhantomData < fn (S) > , }
};
}
