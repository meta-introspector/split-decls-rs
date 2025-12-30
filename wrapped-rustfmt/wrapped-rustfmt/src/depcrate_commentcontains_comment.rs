// Generated macro for contains_comment (function)
macro_rules! Depcrate_commentcontains_comment {
() => {
// Module: crate::comment
// Provides: {"contains_comment"}
// Dependencies: {}
# [doc = " Returns `true` if text contains any comment."] pub (crate) fn contains_comment (text : & str) -> bool { CharClasses :: new (text . chars ()) . any (| (kind , _) | kind . is_comment ()) }
};
}
