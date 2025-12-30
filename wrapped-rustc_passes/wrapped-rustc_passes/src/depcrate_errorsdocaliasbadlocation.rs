// Generated macro for DocAliasBadLocation (struct)
macro_rules! Depcrate_errorsDocAliasBadLocation {
() => {
// Module: crate::errors
// Provides: {"DocAliasBadLocation"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_doc_alias_bad_location)] pub (crate) struct DocAliasBadLocation < 'a > { # [primary_span] pub span : Span , pub attr_str : & 'a str , pub location : & 'a str , }
};
}
