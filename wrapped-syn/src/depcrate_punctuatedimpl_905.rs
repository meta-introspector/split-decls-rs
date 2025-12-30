// Generated macro for impl_905 (impl)
macro_rules! Depcrate_punctuatedimpl_905 {
() => {
// Module: crate::punctuated
// Provides: {"impl_905"}
// Dependencies: {}
impl < T > Iterator for IntoIter < T > { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
