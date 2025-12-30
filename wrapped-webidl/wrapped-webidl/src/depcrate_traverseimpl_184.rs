// Generated macro for impl_184 (impl)
macro_rules! Depcrate_traverseimpl_184 {
() => {
// Module: crate::traverse
// Provides: {"impl_184"}
// Dependencies: {}
impl TraverseType for syn :: ReturnType { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { match self { Self :: Default => { } Self :: Type (_ , ty) => { ty . traverse_type (f) ; } } } }
};
}
