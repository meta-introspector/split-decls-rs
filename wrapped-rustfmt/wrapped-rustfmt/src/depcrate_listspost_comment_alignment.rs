// Generated macro for post_comment_alignment (function)
macro_rules! Depcrate_listspost_comment_alignment {
() => {
// Module: crate::lists
// Provides: {"post_comment_alignment"}
// Dependencies: {}
fn post_comment_alignment (item_max_width : Option < usize > , inner_item_width : usize) -> usize { item_max_width . unwrap_or (0) . saturating_sub (inner_item_width) }
};
}
