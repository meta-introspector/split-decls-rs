// Generated macro for rewrite_comment_inner (function)
macro_rules! Depcrate_commentrewrite_comment_inner {
() => {
// Module: crate::comment
// Provides: {"rewrite_comment_inner"}
// Dependencies: {}
fn rewrite_comment_inner (orig : & str , block_style : bool , style : CommentStyle < '_ > , shape : Shape , config : & Config , is_doc_comment : bool ,) -> RewriteResult { let mut rewriter = CommentRewrite :: new (orig , block_style , shape , config) ; let line_breaks = count_newlines (orig . trim_end ()) ; let lines = orig . lines () . enumerate () . map (| (i , mut line) | { line = trim_end_unless_two_whitespaces (line . trim_start () , is_doc_comment) ; if i == line_breaks && line . ends_with ("*/") && ! line . starts_with ("//") { line = line [.. (line . len () - 2)] . trim_end () ; } line }) . map (| s | left_trim_comment_line (s , & style)) . map (| (line , has_leading_whitespace) | { if orig . starts_with ("/*") && line_breaks == 0 { (line . trim_start () , has_leading_whitespace || config . normalize_comments () ,) } else { (line , has_leading_whitespace || config . normalize_comments ()) } }) ; for (i , (line , has_leading_whitespace)) in lines . enumerate () { if rewriter . handle_line (orig , i , line , has_leading_whitespace , is_doc_comment) { break ; } } Ok (rewriter . finish ()) }
};
}
