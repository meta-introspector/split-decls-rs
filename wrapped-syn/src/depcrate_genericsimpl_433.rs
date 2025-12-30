// Generated macro for impl_433 (impl)
macro_rules! Depcrate_genericsimpl_433 {
() => {
// Module: crate::generics
// Provides: {"impl_433"}
// Dependencies: {}
impl < 'a > Iterator for Lifetimes < 'a > { type Item = & 'a LifetimeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Lifetime (lifetime) = self . 0 . next () ? { Some (lifetime) } else { self . next () } } }
};
}
