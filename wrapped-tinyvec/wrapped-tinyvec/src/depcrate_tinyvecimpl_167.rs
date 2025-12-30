// Generated macro for impl_167 (impl)
macro_rules! Depcrate_tinyvecimpl_167 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'p , A : Array , I : Iterator < Item = A :: Item > > Drop for TinyVecSplice < 'p , A , I > { # [inline] fn drop (& mut self) { for _ in self . by_ref () { } let (lower_bound , _) = self . replacement . size_hint () ; self . parent . reserve (lower_bound) ; for replacement in self . replacement . by_ref () { self . parent . insert (self . removal_end , replacement) ; self . removal_end += 1 ; } } }
};
}
