// Generated macro for impl_34 (impl)
macro_rules! Depcrate_arrayvecimpl_34 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'p , A : Array , I : Iterator < Item = A :: Item > > Iterator for ArrayVecSplice < 'p , A , I > { type Item = A :: Item ; # [inline] fn next (& mut self) -> Option < A :: Item > { if self . removal_start < self . removal_end { match self . replacement . next () { Some (replacement) => { let removed = core :: mem :: replace (& mut self . parent [self . removal_start] , replacement ,) ; self . removal_start += 1 ; Some (removed) } None => { let removed = self . parent . remove (self . removal_start) ; self . removal_end -= 1 ; Some (removed) } } } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = self . len () ; (len , Some (len)) } }
};
}
