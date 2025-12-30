// Generated macro for format_overflow_style (function)
macro_rules! Depcrate_chainsformat_overflow_style {
() => {
// Module: crate::chains
// Provides: {"format_overflow_style"}
// Dependencies: {}
# [doc = " Provides the original input contents from the span"] # [doc = " of a chain element with trailing spaces trimmed."] fn format_overflow_style (span : Span , context : & RewriteContext < '_ >) -> Option < String > { context . snippet_provider . span_to_snippet (span) . map (| s | { s . lines () . map (| l | l . trim_end ()) . collect :: < Vec < _ > > () . join ("\n") }) }
};
}
