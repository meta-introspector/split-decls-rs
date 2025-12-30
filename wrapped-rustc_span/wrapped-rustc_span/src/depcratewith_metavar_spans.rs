// Generated macro for with_metavar_spans (function)
macro_rules! Depcratewith_metavar_spans {
() => {
// Module: crate
// Provides: {"with_metavar_spans"}
// Dependencies: {}
# [inline] pub fn with_metavar_spans < R > (f : impl FnOnce (& MetavarSpansMap) -> R) -> R { with_session_globals (| session_globals | f (& session_globals . metavar_spans)) }
};
}
