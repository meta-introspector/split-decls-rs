// Generated macro for impl_880 (impl)
macro_rules! Depcrate_punctuatedimpl_880 {
() => {
// Module: crate::punctuated
// Provides: {"impl_880"}
// Dependencies: {}
impl < 'a , T , P > DoubleEndedIterator for PairsMut < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . map (Pair :: End) . or_else (| | self . inner . next_back () . map (| (t , p) | Pair :: Punctuated (t , p))) } }
};
}
