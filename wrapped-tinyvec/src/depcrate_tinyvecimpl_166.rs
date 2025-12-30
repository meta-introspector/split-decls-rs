// Generated macro for impl_166 (impl)
macro_rules! Depcrate_tinyvecimpl_166 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'p , A , I > DoubleEndedIterator for TinyVecSplice < 'p , A , I > where A : Array , I : Iterator < Item = A :: Item > + DoubleEndedIterator , { # [inline] fn next_back (& mut self) -> Option < A :: Item > { if self . removal_start < self . removal_end { match self . replacement . next_back () { Some (replacement) => { let removed = core :: mem :: replace (& mut self . parent [self . removal_end - 1] , replacement ,) ; self . removal_end -= 1 ; Some (removed) } None => { let removed = self . parent . remove (self . removal_end - 1) ; self . removal_end -= 1 ; Some (removed) } } } else { None } } }
};
}
