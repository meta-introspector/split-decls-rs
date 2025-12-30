// Generated macro for collect_span (function)
macro_rules! Depcratecollect_span {
() => {
// Module: crate
// Provides: {"collect_span"}
// Dependencies: {}
# [doc = " Converts a [`DiagnosticSpan`] into a [`Replacement`]."] fn collect_span (span : & DiagnosticSpan) -> Option < Replacement > { let snippet = span_to_snippet (span) ; let replacement = span . suggested_replacement . clone () ? ; Some (Replacement { snippet , replacement , }) }
};
}
