// Generated macro for get_comment_end (function)
macro_rules! Depcrate_listsget_comment_end {
() => {
// Module: crate::lists
// Provides: {"get_comment_end"}
// Dependencies: {}
pub (crate) fn get_comment_end (post_snippet : & str , separator : & str , terminator : & str , is_last : bool ,) -> usize { if is_last { return post_snippet . find_uncommented (terminator) . unwrap_or_else (| | post_snippet . len ()) ; } let mut block_open_index = post_snippet . find ("/*") ; if let Some (i) = block_open_index { match post_snippet . find ('/') { Some (j) if j < i => block_open_index = None , _ if post_snippet [.. i] . ends_with ('/') => block_open_index = None , _ => () , } } let newline_index = post_snippet . find ('\n') ; if let Some (separator_index) = post_snippet . find_uncommented (separator) { match (block_open_index , newline_index) { (Some (i) , None) if i > separator_index => separator_index + 1 , (Some (i) , None) => cmp :: max (find_comment_end (& post_snippet [i ..]) . unwrap () + i , separator_index + 1 ,) , (Some (i) , Some (j)) if i < j => cmp :: max (find_comment_end (& post_snippet [i ..]) . unwrap () + i , separator_index + 1 ,) , (_ , Some (j)) if j > separator_index => j + 1 , _ => post_snippet . len () , } } else if let Some (newline_index) = newline_index { newline_index + 1 } else { 0 } }
};
}
