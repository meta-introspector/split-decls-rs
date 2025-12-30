// Generated macro for impl_141 (impl)
macro_rules! Depcrate_iterator_as_exact_size_iteratorimpl_141 {
() => {
// Module: crate::iterator_as_exact_size_iterator
// Provides: {"impl_141"}
// Dependencies: {}
impl < I : Iterator > IteratorAsExactSizeIterator < I > { # [inline] pub (crate) fn new (iter : I) -> Self { let (lower , upper) = iter . size_hint () ; debug_assert_eq ! (Some (lower) , upper , "IteratorAsExactSizeIterator requires size hint lower == upper") ; IteratorAsExactSizeIterator { iter } } }
};
}
