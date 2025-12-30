// Generated macro for span_to_location (function)
macro_rules! Depcrate_source_trackerspan_to_location {
() => {
// Module: crate::source_tracker
// Provides: {"span_to_location"}
// Dependencies: {}
# [doc = " Extract source location from a span"] pub fn span_to_location (span : Span) -> SourceLocation { let line_col : LineColumn = span . start () ; SourceLocation { file : "unknown" . to_string () , line : line_col . line , column : line_col . column , } }
};
}
