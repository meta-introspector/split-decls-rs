// Generated macro for is_last_comment_block (function)
macro_rules! Depcrate_commentis_last_comment_block {
() => {
// Module: crate::comment
// Provides: {"is_last_comment_block"}
// Dependencies: {}
# [doc = " Returns true if the last line of the passed string finishes with a block-comment."] pub (crate) fn is_last_comment_block (s : & str) -> bool { s . trim_end () . ends_with ("*/") }
};
}
