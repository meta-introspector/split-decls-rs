// Generated macro for error_kind_to_snippet_annotation_level (function)
macro_rules! Depcrate_format_report_formattererror_kind_to_snippet_annotation_level {
() => {
// Module: crate::format_report_formatter
// Provides: {"error_kind_to_snippet_annotation_level"}
// Dependencies: {}
fn error_kind_to_snippet_annotation_level (error_kind : & ErrorKind) -> Level { match error_kind { ErrorKind :: LineOverflow (..) | ErrorKind :: TrailingWhitespace | ErrorKind :: IoError (_) | ErrorKind :: ModuleResolutionError (_) | ErrorKind :: ParseError | ErrorKind :: LostComment | ErrorKind :: BadAttr | ErrorKind :: InvalidGlobPattern (_) | ErrorKind :: VersionMismatch => Level :: Error , ErrorKind :: DeprecatedAttr => Level :: Warning , } }
};
}
