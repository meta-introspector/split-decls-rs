// Generated macro for impl_194 (impl)
macro_rules! Depcrate_traverseimpl_194 {
() => {
// Module: crate::traverse
// Provides: {"impl_194"}
// Dependencies: {}
impl TraverseType for syn :: TypePtr { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
