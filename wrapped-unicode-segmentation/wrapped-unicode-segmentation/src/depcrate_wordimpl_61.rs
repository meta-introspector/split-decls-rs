// Generated macro for impl_61 (impl)
macro_rules! Depcrate_wordimpl_61 {
() => {
// Module: crate::word
// Provides: {"impl_61"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for UnicodeWords < 'a > { # [inline] fn next_back (& mut self) -> Option < Self :: Item > { match & mut self . inner { WordsIter :: Ascii (i) => i . next_back () , WordsIter :: Unicode (i) => i . next_back () , } } }
};
}
