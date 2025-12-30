// Generated macro for impl_8 (impl)
macro_rules! Depcrate_backtraceimpl_8 {
() => {
// Module: crate::backtrace
// Provides: {"impl_8"}
// Dependencies: {}
impl SpanTraceStatus { # [doc = " Formatting a SpanTrace is not supported, likely because there is no"] # [doc = " ErrorLayer or the ErrorLayer is from a different version of"] # [doc = " tracing_error"] pub const UNSUPPORTED : SpanTraceStatus = SpanTraceStatus (SpanTraceStatusInner :: Unsupported) ; # [doc = " The SpanTrace is empty, likely because it was captured outside of any"] # [doc = " `span`s"] pub const EMPTY : SpanTraceStatus = SpanTraceStatus (SpanTraceStatusInner :: Empty) ; # [doc = " A span trace has been captured and the `SpanTrace` should print"] # [doc = " reasonable information when rendered."] pub const CAPTURED : SpanTraceStatus = SpanTraceStatus (SpanTraceStatusInner :: Captured) ; }
};
}
