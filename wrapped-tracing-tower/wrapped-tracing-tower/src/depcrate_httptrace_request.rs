// Generated macro for trace_request (function)
macro_rules! Depcrate_httptrace_request {
() => {
// Module: crate::http
// Provides: {"trace_request"}
// Dependencies: {}
# [inline] pub fn trace_request < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! (tracing :: Level :: TRACE , "request" , method = ? req . method () , uri = ? req . uri () , version = ? req . version () , headers = ? req . headers () ,) }
};
}
