// Generated macro for impl_183 (impl)
macro_rules! Depcrate_tinyvecimpl_183 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_183"}
// Dependencies: {}
impl < A : Array > DoubleEndedIterator for TinyVecIterator < A > { impl_mirrored ! { type Mirror = TinyVecIterator ; # [inline] fn next_back (self : & mut Self) -> Option < Self :: Item >; # [inline] fn nth_back (self : & mut Self , n : usize) -> Option < Self :: Item >; } }
};
}
