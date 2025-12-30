// Generated macro for impl_901 (impl)
macro_rules! Depcrate_punctuatedimpl_901 {
() => {
// Module: crate::punctuated
// Provides: {"impl_901"}
// Dependencies: {}
impl < 'a , T , P > Iterator for PrivateIter < 'a , T , P > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| pair | & pair . 0) . or_else (| | self . last . next ()) } }
};
}
