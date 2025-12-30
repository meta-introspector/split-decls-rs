// Generated macro for impl_161 (impl)
macro_rules! Depcrate_tinyvecimpl_161 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_161"}
// Dependencies: {}
impl < 'p , A : Array > DoubleEndedIterator for TinyVecDrain < 'p , A > { impl_mirrored ! { type Mirror = TinyVecDrain ; # [inline] fn next_back (self : & mut Self) -> Option < Self :: Item >; # [inline] fn nth_back (self : & mut Self , n : usize) -> Option < Self :: Item >; } }
};
}
