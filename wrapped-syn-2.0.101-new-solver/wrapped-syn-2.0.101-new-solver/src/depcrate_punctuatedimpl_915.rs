// Generated macro for impl_915 (impl)
macro_rules! Depcrate_punctuatedimpl_915 {
() => {
// Module: crate::punctuated
// Provides: {"impl_915"}
// Dependencies: {}
impl < 'a , T , P > DoubleEndedIterator for PrivateIterMut < 'a , T , P > { fn next_back (& mut self) -> Option < Self :: Item > { self . last . next () . or_else (| | self . inner . next_back () . map (| pair | & mut pair . 0)) } }
};
}
