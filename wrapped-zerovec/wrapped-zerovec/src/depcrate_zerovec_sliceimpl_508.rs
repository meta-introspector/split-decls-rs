// Generated macro for impl_508 (impl)
macro_rules! Depcrate_zerovec_sliceimpl_508 {
() => {
// Module: crate::zerovec::slice
// Provides: {"impl_508"}
// Dependencies: {}
impl < 'a , T : AsULE > Iterator for ZeroSliceIter < 'a , T > { type Item = T ; fn next (& mut self) -> Option < T > { self . 0 . next () . copied () . map (T :: from_unaligned) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . size_hint () } }
};
}
