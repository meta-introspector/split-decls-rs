// Generated macro for error_kind_to_snippet_annotation_type (function)
macro_rules! Depcrate_format_report_formattererror_kind_to_snippet_annotation_type {
() => {
// Module: crate::format_report_formatter
// Provides: {"error_kind_to_snippet_annotation_type"}
// Dependencies: {}
fn error_kind_to_snippet_annotation_type (error_kind : & ErrorKind) -> AnnotationType { match error_kind { ErrorKind :: LineOverflow (..) | ErrorKind :: TrailingWhitespace | ErrorKind :: IoError (_) | ErrorKind :: ModuleResolutionError (_) | ErrorKind :: ParseError | ErrorKind :: LostComment | ErrorKind :: BadAttr | ErrorKind :: InvalidGlobPattern (_) | ErrorKind :: VersionMismatch => AnnotationType :: Error , ErrorKind :: DeprecatedAttr => AnnotationType :: Warning , } }
};
}
