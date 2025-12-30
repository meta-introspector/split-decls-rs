// Generated macro for impl_437 (impl)
macro_rules! Depcrate_genericsimpl_437 {
() => {
// Module: crate::generics
// Provides: {"impl_437"}
// Dependencies: {}
impl < 'a > Iterator for TypeParams < 'a > { type Item = & 'a TypeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Type (type_param) = self . 0 . next () ? { Some (type_param) } else { self . next () } } }
};
}
