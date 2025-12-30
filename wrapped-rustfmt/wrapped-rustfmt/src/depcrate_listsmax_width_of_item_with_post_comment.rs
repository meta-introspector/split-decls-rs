// Generated macro for max_width_of_item_with_post_comment (function)
macro_rules! Depcrate_listsmax_width_of_item_with_post_comment {
() => {
// Module: crate::lists
// Provides: {"max_width_of_item_with_post_comment"}
// Dependencies: {}
fn max_width_of_item_with_post_comment < I , T > (items : & I , i : usize , overhead : usize , max_budget : usize ,) -> usize where I : IntoIterator < Item = T > + Clone , T : AsRef < ListItem > , { let mut max_width = 0 ; let mut first = true ; for item in items . clone () . into_iter () . skip (i) { let item = item . as_ref () ; let inner_item_width = unicode_str_width (item . inner_as_ref ()) ; if ! first && (item . is_different_group () || item . post_comment . is_none () || inner_item_width + overhead > max_budget) { return max_width ; } if max_width < inner_item_width { max_width = inner_item_width ; } if item . new_lines { return max_width ; } first = false ; } max_width }
};
}
