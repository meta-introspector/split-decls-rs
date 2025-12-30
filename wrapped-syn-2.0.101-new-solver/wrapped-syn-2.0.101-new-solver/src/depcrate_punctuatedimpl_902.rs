// Generated macro for impl_902 (impl)
macro_rules! Depcrate_punctuatedimpl_902 {
() => {
// Module: crate::punctuated
// Provides: {"impl_902"}
// Dependencies: {}
impl < 'a , T , P > DoubleEndedIterator for PrivateIter < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . or_else (| | self . inner . next_back () . map (| pair | & pair . 0)) } }
};
}
