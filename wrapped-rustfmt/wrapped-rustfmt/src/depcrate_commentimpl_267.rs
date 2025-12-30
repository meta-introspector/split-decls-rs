// Generated macro for impl_267 (impl)
macro_rules! Depcrate_commentimpl_267 {
() => {
// Module: crate::comment
// Provides: {"impl_267"}
// Dependencies: {}
impl < 'a > Iterator for UngroupedCommentCodeSlices < 'a > { type Item = (CodeCharKind , usize , & 'a str) ; fn next (& mut self) -> Option < Self :: Item > { let (kind , (start_idx , _)) = self . iter . next () ? ; match kind { FullCodeCharKind :: Normal | FullCodeCharKind :: InString => { while let Some (& (char_kind , _)) = self . iter . peek () { if char_kind . is_comment () { break ; } let _ = self . iter . next () ; } } FullCodeCharKind :: StartComment => { loop { match self . iter . next () { Some ((kind , ..)) if kind . inside_comment () => continue , _ => break , } } } _ => panic ! () , } let slice = match self . iter . peek () { Some (& (_ , (end_idx , _))) => & self . slice [start_idx .. end_idx] , None => & self . slice [start_idx ..] , } ; Some ((if kind . is_comment () { CodeCharKind :: Comment } else { CodeCharKind :: Normal } , start_idx , slice ,)) } }
};
}
