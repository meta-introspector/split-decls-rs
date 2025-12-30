// Generated macro for extract_comment (function)
macro_rules! Depcrate_exprextract_comment {
() => {
// Module: crate::expr
// Provides: {"extract_comment"}
// Dependencies: {}
fn extract_comment (span : Span , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { match rewrite_missing_comment (span , shape , context) { Ok (ref comment) if ! comment . is_empty () => Some (format ! ("{indent}{comment}{indent}" , indent = shape . indent . to_string_with_newline (context . config))) , _ => None , } }
};
}
