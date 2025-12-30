// Generated macro for comment_style (function)
macro_rules! Depcrate_commentcomment_style {
() => {
// Module: crate::comment
// Provides: {"comment_style"}
// Dependencies: {}
pub (crate) fn comment_style (orig : & str , normalize_comments : bool) -> CommentStyle < '_ > { if ! normalize_comments { if orig . starts_with ("/**") && ! orig . starts_with ("/**/") { CommentStyle :: DoubleBullet } else if orig . starts_with ("/*!") { CommentStyle :: Exclamation } else if orig . starts_with ("/*") { CommentStyle :: SingleBullet } else if orig . starts_with ("///") && orig . chars () . nth (3) . map_or (true , | c | c != '/') { CommentStyle :: TripleSlash } else if orig . starts_with ("//!") { CommentStyle :: Doc } else if is_custom_comment (orig) { CommentStyle :: Custom (custom_opener (orig)) } else { CommentStyle :: DoubleSlash } } else if (orig . starts_with ("///") && orig . chars () . nth (3) . map_or (true , | c | c != '/')) || (orig . starts_with ("/**") && ! orig . starts_with ("/**/")) { CommentStyle :: TripleSlash } else if orig . starts_with ("//!") || orig . starts_with ("/*!") { CommentStyle :: Doc } else if is_custom_comment (orig) { CommentStyle :: Custom (custom_opener (orig)) } else { CommentStyle :: DoubleSlash } }
};
}
