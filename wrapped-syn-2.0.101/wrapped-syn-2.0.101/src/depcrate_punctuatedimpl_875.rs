// Generated macro for impl_875 (impl)
macro_rules! Depcrate_punctuatedimpl_875 {
() => {
// Module: crate::punctuated
// Provides: {"impl_875"}
// Dependencies: {}
impl < 'a , T , P > DoubleEndedIterator for Pairs < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
};
}
