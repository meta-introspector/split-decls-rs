// Generated macro for format_trace (function)
macro_rules! Depcrateformat_trace {
() => {
// Module: crate
// Provides: {"format_trace"}
// Dependencies: {}
# [doc = " Format a log record as a trace event in the current span."] pub fn format_trace (record : & log :: Record < '_ >) -> io :: Result < () > { dispatch_record (record) ; Ok (()) }
};
}
