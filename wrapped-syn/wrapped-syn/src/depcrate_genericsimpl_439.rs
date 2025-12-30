// Generated macro for impl_439 (impl)
macro_rules! Depcrate_genericsimpl_439 {
() => {
// Module: crate::generics
// Provides: {"impl_439"}
// Dependencies: {}
impl < 'a > Iterator for TypeParamsMut < 'a > { type Item = & 'a mut TypeParam ; fn next (& mut self) -> Option < Self :: Item > { if let GenericParam :: Type (type_param) = self . 0 . next () ? { Some (type_param) } else { self . next () } } }
};
}
