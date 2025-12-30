// Generated macro for ExtractSpanTrace (trait)
macro_rules! Depcrate_errorExtractSpanTrace {
() => {
// Module: crate::error
// Provides: {"ExtractSpanTrace"}
// Dependencies: {}
# [doc = " A trait for extracting SpanTraces created by `in_current_span()` from `dyn"] # [doc = " Error` trait objects"] # [cfg_attr (docsrs , doc (cfg (feature = "traced-error")))] pub trait ExtractSpanTrace { # [doc = " Attempts to downcast to a `TracedError` and return a reference to its"] # [doc = " SpanTrace"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tracing_error::ExtractSpanTrace;"] # [doc = " use std::error::Error;"] # [doc = ""] # [doc = " fn print_span_trace(e: &(dyn Error + 'static)) {"] # [doc = "     let span_trace = e.span_trace();"] # [doc = "     if let Some(span_trace) = span_trace {"] # [doc = "         println!(\"{}\", span_trace);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] fn span_trace (& self) -> Option < & SpanTrace > ; }
};
}
