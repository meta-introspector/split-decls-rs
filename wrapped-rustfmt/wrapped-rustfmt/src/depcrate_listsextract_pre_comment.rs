// Generated macro for extract_pre_comment (function)
macro_rules! Depcrate_listsextract_pre_comment {
() => {
// Module: crate::lists
// Provides: {"extract_pre_comment"}
// Dependencies: {}
pub (crate) fn extract_pre_comment (pre_snippet : & str) -> (Option < String > , ListItemCommentStyle) { let trimmed_pre_snippet = pre_snippet . trim () ; let starts_with_block_comment = trimmed_pre_snippet . starts_with ("/*") ; let ends_with_block_comment = trimmed_pre_snippet . ends_with ("*/") ; let starts_with_single_line_comment = trimmed_pre_snippet . starts_with ("//") ; if ends_with_block_comment { let comment_end = pre_snippet . rfind (| c | c == '/') . unwrap () ; if pre_snippet [comment_end ..] . contains ('\n') { (Some (trimmed_pre_snippet . to_owned ()) , ListItemCommentStyle :: DifferentLine ,) } else { (Some (trimmed_pre_snippet . to_owned ()) , ListItemCommentStyle :: SameLine ,) } } else if starts_with_single_line_comment || starts_with_block_comment { (Some (trimmed_pre_snippet . to_owned ()) , ListItemCommentStyle :: DifferentLine ,) } else { (None , ListItemCommentStyle :: None) } }
};
}
