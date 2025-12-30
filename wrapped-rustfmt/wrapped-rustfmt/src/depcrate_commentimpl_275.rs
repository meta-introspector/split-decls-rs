// Generated macro for impl_275 (impl)
macro_rules! Depcrate_commentimpl_275 {
() => {
// Module: crate::comment
// Provides: {"impl_275"}
// Dependencies: {}
impl < 'a > CommentReducer < 'a > { fn new (comment : & 'a str) -> CommentReducer < 'a > { let is_block = comment . starts_with ("/*") ; let comment = remove_comment_header (comment) ; CommentReducer { is_block , at_start_line : false , iter : comment . chars () , } } }
};
}
