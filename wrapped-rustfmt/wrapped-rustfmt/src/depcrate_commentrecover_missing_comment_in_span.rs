// Generated macro for recover_missing_comment_in_span (function)
macro_rules! Depcrate_commentrecover_missing_comment_in_span {
() => {
// Module: crate::comment
// Provides: {"recover_missing_comment_in_span"}
// Dependencies: {}
# [doc = " Recover the missing comments in the specified span, if available."] # [doc = " The layout of the comments will be preserved as long as it does not break the code"] # [doc = " and its total width does not exceed the max width."] pub (crate) fn recover_missing_comment_in_span (span : Span , shape : Shape , context : & RewriteContext < '_ > , used_width : usize ,) -> RewriteResult { let missing_comment = rewrite_missing_comment (span , shape , context) ? ; if missing_comment . is_empty () { Ok (String :: new ()) } else { let missing_snippet = context . snippet (span) ; let pos = missing_snippet . find ('/') . unknown_error () ? ; let total_width = missing_comment . len () + used_width + 1 ; let force_new_line_before_comment = missing_snippet [.. pos] . contains ('\n') || total_width > context . config . max_width () ; let sep = if force_new_line_before_comment { shape . indent . to_string_with_newline (context . config) } else { Cow :: from (" ") } ; Ok (format ! ("{sep}{missing_comment}")) } }
};
}
