macro_rules! debug_request {
    () => {
        # [inline] pub fn debug_request < A > (req : & http :: Request < A >) -> tracing :: Span { tracing :: span ! (tracing :: Level :: DEBUG , "request" , method = ? req . method () , uri = ? req . uri () , version = ? req . version () ,) }
    };
}

debug_request!();