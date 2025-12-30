// Generated macro for impl_185 (impl)
macro_rules! Depcrate_traverseimpl_185 {
() => {
// Module: crate::traverse
// Provides: {"impl_185"}
// Dependencies: {}
impl TraverseType for syn :: TypeGroup { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
