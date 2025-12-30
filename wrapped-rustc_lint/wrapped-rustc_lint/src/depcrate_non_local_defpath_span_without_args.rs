// Generated macro for path_span_without_args (function)
macro_rules! Depcrate_non_local_defpath_span_without_args {
() => {
// Module: crate::non_local_def
// Provides: {"path_span_without_args"}
// Dependencies: {}
# [doc = " Return for a given `Path` the span until the last args"] fn path_span_without_args (path : & Path < '_ >) -> Span { if let Some (args) = & path . segments . last () . unwrap () . args { path . span . until (args . span_ext) } else { path . span } }
};
}
