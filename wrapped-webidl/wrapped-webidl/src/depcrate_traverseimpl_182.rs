// Generated macro for impl_182 (impl)
macro_rules! Depcrate_traverseimpl_182 {
() => {
// Module: crate::traverse
// Provides: {"impl_182"}
// Dependencies: {}
impl TraverseType for syn :: TypeArray { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
