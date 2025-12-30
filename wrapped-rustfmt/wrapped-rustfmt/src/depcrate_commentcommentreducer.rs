// Generated macro for CommentReducer (struct)
macro_rules! Depcrate_commentCommentReducer {
() => {
// Module: crate::comment
// Provides: {"CommentReducer"}
// Dependencies: {}
# [doc = " Iterator over the 'payload' characters of a comment."] # [doc = " It skips whitespace, comment start/end marks, and '*' at the beginning of lines."] # [doc = " The comment must be one comment, ie not more than one start mark (no multiple line comments,"] # [doc = " for example)."] struct CommentReducer < 'a > { is_block : bool , at_start_line : bool , iter : std :: str :: Chars < 'a > , }
};
}
