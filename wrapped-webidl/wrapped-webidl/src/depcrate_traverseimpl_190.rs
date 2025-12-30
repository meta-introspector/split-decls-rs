// Generated macro for impl_190 (impl)
macro_rules! Depcrate_traverseimpl_190 {
() => {
// Module: crate::traverse
// Provides: {"impl_190"}
// Dependencies: {}
impl TraverseType for syn :: AngleBracketedGenericArguments { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { for ty in self . args . iter () { ty . traverse_type (f) ; } } }
};
}
