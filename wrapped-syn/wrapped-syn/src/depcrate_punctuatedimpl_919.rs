// Generated macro for impl_919 (impl)
macro_rules! Depcrate_punctuatedimpl_919 {
() => {
// Module: crate::punctuated
// Provides: {"impl_919"}
// Dependencies: {}
impl < 'a , T , P > DoubleEndedIterator for PrivateIter < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . or_else (| | self . inner . next_back () . map (| pair | & pair . 0)) } }
};
}
