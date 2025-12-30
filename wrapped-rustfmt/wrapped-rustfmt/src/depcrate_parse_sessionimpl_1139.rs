// Generated macro for impl_1139 (impl)
macro_rules! Depcrate_parse_sessionimpl_1139 {
() => {
// Module: crate::parse::session
// Provides: {"impl_1139"}
// Dependencies: {}
impl Emitter for SilentOnIgnoredFilesEmitter { fn source_map (& self) -> Option < & SourceMap > { None } fn emit_diagnostic (& mut self , diag : DiagInner , registry : & Registry) { if diag . level () == DiagnosticLevel :: Fatal { return self . handle_non_ignoreable_error (diag , registry) ; } if let Some (primary_span) = & diag . span . primary_span () { let file_name = self . source_map . span_to_filename (* primary_span) ; if let rustc_span :: FileName :: Real (rustc_span :: RealFileName :: LocalPath (ref path)) = file_name { if self . ignore_path_set . is_match (& FileName :: Real (path . to_path_buf ())) { if ! self . has_non_ignorable_parser_errors { self . can_reset . store (true , Ordering :: Release) ; } return ; } } ; } self . handle_non_ignoreable_error (diag , registry) ; } }
};
}
