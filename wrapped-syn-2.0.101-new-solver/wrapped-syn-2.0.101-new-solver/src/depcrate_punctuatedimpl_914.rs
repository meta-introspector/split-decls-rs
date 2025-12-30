// Generated macro for impl_914 (impl)
macro_rules! Depcrate_punctuatedimpl_914 {
() => {
// Module: crate::punctuated
// Provides: {"impl_914"}
// Dependencies: {}
impl < 'a , T , P > Iterator for PrivateIterMut < 'a , T , P > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & mut pair . 0) . or_else (| | self . last . next ()) } }
};
}
