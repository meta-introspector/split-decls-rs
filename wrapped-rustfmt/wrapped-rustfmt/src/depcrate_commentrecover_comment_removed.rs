// Generated macro for recover_comment_removed (function)
macro_rules! Depcrate_commentrecover_comment_removed {
() => {
// Module: crate::comment
// Provides: {"recover_comment_removed"}
// Dependencies: {}
# [doc = " Checks is `new` didn't miss any comment from `span`, if it removed any, return previous text"] pub (crate) fn recover_comment_removed (new : String , span : Span , context : & RewriteContext < '_ > ,) -> String { let snippet = context . snippet (span) ; if snippet != new && changed_comment_content (snippet , & new) { if context . config . error_on_unformatted () { context . report . append (context . psess . span_to_filename (span) , vec ! [FormattingError :: from_span (span , context . psess , ErrorKind :: LostComment ,)] ,) ; } snippet . to_owned () } else { new } }
};
}
