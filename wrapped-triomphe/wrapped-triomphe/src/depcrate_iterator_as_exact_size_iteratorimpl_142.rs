// Generated macro for impl_142 (impl)
macro_rules! Depcrate_iterator_as_exact_size_iteratorimpl_142 {
() => {
// Module: crate::iterator_as_exact_size_iterator
// Provides: {"impl_142"}
// Dependencies: {}
impl < I : Iterator > Iterator for IteratorAsExactSizeIterator < I > { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . iter . next () } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
};
}
