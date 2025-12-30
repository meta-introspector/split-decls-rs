// Generated macro for get_missing_param_comments (function)
macro_rules! Depcrate_itemsget_missing_param_comments {
() => {
// Module: crate::items
// Provides: {"get_missing_param_comments"}
// Dependencies: {}
# [doc = " Recover any missing comments between the param and the type."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " A 2-len tuple with the comment before the colon in first position, and the comment after the"] # [doc = " colon in second position."] fn get_missing_param_comments (context : & RewriteContext < '_ > , pat_span : Span , ty_span : Span , shape : Shape ,) -> (String , String) { let missing_comment_span = mk_sp (pat_span . hi () , ty_span . lo ()) ; let span_before_colon = { let missing_comment_span_hi = context . snippet_provider . span_before (missing_comment_span , ":") ; mk_sp (pat_span . hi () , missing_comment_span_hi) } ; let span_after_colon = { let missing_comment_span_lo = context . snippet_provider . span_after (missing_comment_span , ":") ; mk_sp (missing_comment_span_lo , ty_span . lo ()) } ; let comment_before_colon = rewrite_missing_comment (span_before_colon , shape , context) . ok () . filter (| comment | ! comment . is_empty ()) . map_or (String :: new () , | comment | format ! (" {}" , comment)) ; let comment_after_colon = rewrite_missing_comment (span_after_colon , shape , context) . ok () . filter (| comment | ! comment . is_empty ()) . map_or (String :: new () , | comment | format ! ("{} " , comment)) ; (comment_before_colon , comment_after_colon) }
};
}
