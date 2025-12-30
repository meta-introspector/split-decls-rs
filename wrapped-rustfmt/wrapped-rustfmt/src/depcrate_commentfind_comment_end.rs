// Generated macro for find_comment_end (function)
macro_rules! Depcrate_commentfind_comment_end {
() => {
// Module: crate::comment
// Provides: {"find_comment_end"}
// Dependencies: {}
pub (crate) fn find_comment_end (s : & str) -> Option < usize > { let mut iter = CharClasses :: new (s . char_indices ()) ; for (kind , (i , _c)) in & mut iter { if kind == FullCodeCharKind :: Normal || kind == FullCodeCharKind :: InString { return Some (i) ; } } if iter . status == CharClassesStatus :: Normal { Some (s . len ()) } else { None } }
};
}
