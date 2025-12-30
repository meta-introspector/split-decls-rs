// Generated macro for impl_931 (impl)
macro_rules! Depcrate_punctuatedimpl_931 {
() => {
// Module: crate::punctuated
// Provides: {"impl_931"}
// Dependencies: {}
impl < 'a , T , P > Iterator for PrivateIterMut < 'a , T , P > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & mut pair . 0) . or_else (| | self . last . next ()) } }
};
}
