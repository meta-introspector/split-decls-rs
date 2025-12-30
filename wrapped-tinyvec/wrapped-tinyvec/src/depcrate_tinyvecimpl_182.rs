// Generated macro for impl_182 (impl)
macro_rules! Depcrate_tinyvecimpl_182 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_182"}
// Dependencies: {}
impl < A : Array > Iterator for TinyVecIterator < A > { type Item = A :: Item ; impl_mirrored ! { type Mirror = TinyVecIterator ; # [inline] fn next (self : & mut Self) -> Option < Self :: Item >; # [inline (always)] # [must_use] fn size_hint (self : & Self) -> (usize , Option < usize >) ; # [inline (always)] fn count (self : Self) -> usize ; # [inline] fn last (self : Self) -> Option < Self :: Item >; # [inline] fn nth (self : & mut Self , n : usize) -> Option < A :: Item >; } }
};
}
