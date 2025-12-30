// Generated macro for impl_928 (impl)
macro_rules! Depcrate_punctuatedimpl_928 {
() => {
// Module: crate::punctuated
// Provides: {"impl_928"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = & 'a mut T ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () } fn size_hint (& self) -> (usize , Option < usize >) { (self . len () , Some (self . len ())) } }
};
}
