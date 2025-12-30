// Generated macro for total_item_width (function)
macro_rules! Depcrate_liststotal_item_width {
() => {
// Module: crate::lists
// Provides: {"total_item_width"}
// Dependencies: {}
pub (crate) fn total_item_width (item : & ListItem) -> usize { comment_len (item . pre_comment . as_ref () . map (| x | & (* x) [..])) + comment_len (item . post_comment . as_ref () . map (| x | & (* x) [..])) + item . item . as_ref () . map_or (0 , | s | unicode_str_width (s)) }
};
}
