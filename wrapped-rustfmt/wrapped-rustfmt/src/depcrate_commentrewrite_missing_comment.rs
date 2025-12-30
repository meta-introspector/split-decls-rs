// Generated macro for rewrite_missing_comment (function)
macro_rules! Depcrate_commentrewrite_missing_comment {
() => {
// Module: crate::comment
// Provides: {"rewrite_missing_comment"}
// Dependencies: {}
# [doc = " Given the span, rewrite the missing comment inside it if available."] # [doc = " Note that the given span must only include comments (or leading/trailing whitespaces)."] pub (crate) fn rewrite_missing_comment (span : Span , shape : Shape , context : & RewriteContext < '_ > ,) -> RewriteResult { let missing_snippet = context . snippet (span) ; let trimmed_snippet = missing_snippet . trim () ; let pos = trimmed_snippet . find ('/') ; if ! trimmed_snippet . is_empty () && pos . is_some () { rewrite_comment (trimmed_snippet , false , shape , context . config) } else { Ok (String :: new ()) } }
};
}
