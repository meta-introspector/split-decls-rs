// Generated macro for comment_len (function)
macro_rules! Depcrate_listscomment_len {
() => {
// Module: crate::lists
// Provides: {"comment_len"}
// Dependencies: {}
fn comment_len (comment : Option < & str >) -> usize { match comment { Some (s) => { let text_len = s . trim () . len () ; if text_len > 0 { text_len + 6 } else { text_len } } None => 0 , } }
};
}
