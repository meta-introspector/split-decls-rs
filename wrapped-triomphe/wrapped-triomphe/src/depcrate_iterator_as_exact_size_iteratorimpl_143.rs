// Generated macro for impl_143 (impl)
macro_rules! Depcrate_iterator_as_exact_size_iteratorimpl_143 {
() => {
// Module: crate::iterator_as_exact_size_iterator
// Provides: {"impl_143"}
// Dependencies: {}
impl < I : Iterator > ExactSizeIterator for IteratorAsExactSizeIterator < I > { # [inline] fn len (& self) -> usize { let (lower , upper) = self . iter . size_hint () ; debug_assert_eq ! (Some (lower) , upper , "IteratorAsExactSizeIterator requires size hint lower == upper") ; lower } }
};
}
