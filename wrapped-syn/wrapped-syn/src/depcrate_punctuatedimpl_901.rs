// Generated macro for impl_901 (impl)
macro_rules! Depcrate_punctuatedimpl_901 {
() => {
// Module: crate::punctuated
// Provides: {"impl_901"}
// Dependencies: {}
impl < T , P > DoubleEndedIterator for IntoPairs < T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
};
}
