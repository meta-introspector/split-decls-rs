// Generated macro for rewrite_comments_before_after_where (function)
macro_rules! Depcrate_itemsrewrite_comments_before_after_where {
() => {
// Module: crate::items
// Provides: {"rewrite_comments_before_after_where"}
// Dependencies: {}
fn rewrite_comments_before_after_where (context : & RewriteContext < '_ > , span_before_where : Span , span_after_where : Span , shape : Shape ,) -> Result < (String , String) , RewriteError > { let before_comment = rewrite_missing_comment (span_before_where , shape , context) ? ; let after_comment = rewrite_missing_comment (span_after_where , shape . block_indent (context . config . tab_spaces ()) , context ,) ? ; Ok ((before_comment , after_comment)) }
};
}
