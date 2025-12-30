// Generated macro for impl_192 (impl)
macro_rules! Depcrate_traverseimpl_192 {
() => {
// Module: crate::traverse
// Provides: {"impl_192"}
// Dependencies: {}
impl TraverseType for syn :: AssocType { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . ty . traverse_type (f) ; } }
};
}
