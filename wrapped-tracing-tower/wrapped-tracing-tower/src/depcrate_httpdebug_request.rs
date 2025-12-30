// Generated macro for debug_request (function)
macro_rules! Depcrate_httpdebug_request {
() => {
// Module: crate::http
// Provides: {"debug_request"}
// Dependencies: {}
# [inline] pub fn debug_request < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! (tracing :: Level :: DEBUG , "request" , method = ? req . method () , uri = ? req . uri () , version = ? req . version () ,) }
};
}
