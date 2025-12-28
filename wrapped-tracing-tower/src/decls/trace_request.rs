macro_rules! trace_request {
    () => {
        # [inline] pub fn trace_request < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! (tracing :: Level :: TRACE , "request" , method = ? req . method () , uri = ? req . uri () , version = ? req . version () , headers = ? req . headers () ,) }
    };
}

trace_request!();