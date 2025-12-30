// Generated macro for impl_423 (impl)
macro_rules! Depcrate_genericsimpl_423 {
() => {
// Module: crate::generics
// Provides: {"impl_423"}
// Dependencies: {}
impl < 'a > Iterator for LifetimesMut < 'a > { type Item = & 'a mut LifetimeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Lifetime (lifetime) = self . 0 . next () ? { Some (lifetime) } else { self . next () } } }
};
}
