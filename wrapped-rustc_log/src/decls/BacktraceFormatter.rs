macro_rules! BacktraceFormatter {
    () => {
        struct BacktraceFormatter { backtrace_target : String , }
    };
}

BacktraceFormatter!()