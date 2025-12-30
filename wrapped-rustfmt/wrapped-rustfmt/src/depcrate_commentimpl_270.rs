// Generated macro for impl_270 (impl)
macro_rules! Depcrate_commentimpl_270 {
() => {
// Module: crate::comment
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'a > Iterator for CommentCodeSlices < 'a > { type Item = (CodeCharKind , usize , & 'a str) ; fn next (& mut self) -> Option < Self :: Item > { if self . last_slice_end == self . slice . len () { return None ; } let mut sub_slice_end = self . last_slice_end ; let mut first_whitespace = None ; let subslice = & self . slice [self . last_slice_end ..] ; let mut iter = CharClasses :: new (subslice . char_indices ()) ; for (kind , (i , c)) in & mut iter { let is_comment_connector = self . last_slice_kind == CodeCharKind :: Normal && & subslice [.. 2] == "//" && [' ' , '\t'] . contains (& c) ; if is_comment_connector && first_whitespace . is_none () { first_whitespace = Some (i) ; } if kind . to_codecharkind () == self . last_slice_kind && ! is_comment_connector { let last_index = match first_whitespace { Some (j) => j , None => i , } ; sub_slice_end = self . last_slice_end + last_index ; break ; } if ! is_comment_connector { first_whitespace = None ; } } if let (None , true) = (iter . next () , sub_slice_end == self . last_slice_end) { sub_slice_end = match first_whitespace { Some (i) => self . last_slice_end + i , None => self . slice . len () , } ; } let kind = match self . last_slice_kind { CodeCharKind :: Comment => CodeCharKind :: Normal , CodeCharKind :: Normal => CodeCharKind :: Comment , } ; let res = (kind , self . last_slice_end , & self . slice [self . last_slice_end .. sub_slice_end] ,) ; self . last_slice_end = sub_slice_end ; self . last_slice_kind = kind ; Some (res) } }
};
}
