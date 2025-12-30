// Generated macro for impl_64 (impl)
macro_rules! Depcrate_wordimpl_64 {
() => {
// Module: crate::word
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for UnicodeWordIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match & mut self . inner { IndicesIter :: Ascii (i) => i . next_back () , IndicesIter :: Unicode (i) => i . next_back () , } } }
};
}
