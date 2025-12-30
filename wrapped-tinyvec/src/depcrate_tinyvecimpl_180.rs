// Generated macro for impl_180 (impl)
macro_rules! Depcrate_tinyvecimpl_180 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_180"}
// Dependencies: {}
impl < A : Array > TinyVecIterator < A > { impl_mirrored ! { type Mirror = TinyVecIterator ; # [doc = " Returns the remaining items of this iterator as a slice."] # [inline] # [must_use] pub fn as_slice (self : & Self) -> & [A :: Item] ; } }
};
}
