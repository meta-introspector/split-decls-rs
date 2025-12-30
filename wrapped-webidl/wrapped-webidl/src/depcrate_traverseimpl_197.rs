// Generated macro for impl_197 (impl)
macro_rules! Depcrate_traverseimpl_197 {
() => {
// Module: crate::traverse
// Provides: {"impl_197"}
// Dependencies: {}
impl TraverseType for syn :: TypeSlice { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
