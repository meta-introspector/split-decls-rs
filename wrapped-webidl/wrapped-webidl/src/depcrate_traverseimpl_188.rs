// Generated macro for impl_188 (impl)
macro_rules! Depcrate_traverseimpl_188 {
() => {
// Module: crate::traverse
// Provides: {"impl_188"}
// Dependencies: {}
impl TraverseType for syn :: QSelf { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . ty . traverse_type (f) ; } }
};
}
