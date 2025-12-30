// Generated macro for impl_918 (impl)
macro_rules! Depcrate_punctuatedimpl_918 {
() => {
// Module: crate::punctuated
// Provides: {"impl_918"}
// Dependencies: {}
impl < 'a , T , P > Iterator for PrivateIter < 'a , T , P > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & pair . 0) . or_else (| | self . last . next ()) } }
};
}
