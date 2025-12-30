// Generated macro for impl_1124 (impl)
macro_rules! Depcrate_parse_sessionimpl_1124 {
() => {
// Module: crate::parse::session
// Provides: {"impl_1124"}
// Dependencies: {}
impl SilentOnIgnoredFilesEmitter { fn handle_non_ignoreable_error (& mut self , diag : DiagInner , registry : & Registry) { self . has_non_ignorable_parser_errors = true ; self . can_reset . store (false , Ordering :: Release) ; self . emitter . emit_diagnostic (diag , registry) ; } }
};
}
