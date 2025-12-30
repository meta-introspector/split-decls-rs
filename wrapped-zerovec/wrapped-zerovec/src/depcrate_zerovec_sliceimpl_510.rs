// Generated macro for impl_510 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_510 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_510"}
// Dependencies: {}
impl < 'a , T : AsULE > DoubleEndedIterator for ZeroSliceIter < 'a , T > { fn next_back (& mut self) -> Option < T > { self . 0 . next_back () . copied () . map (T :: from_unaligned) } }
};
}
