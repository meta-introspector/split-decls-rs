// Generated macro for impl_195 (impl)
macro_rules! Depcrate_traverseimpl_195 {
() => {
// Module: crate::traverse
// Provides: {"impl_195"}
// Dependencies: {}
impl TraverseType for syn :: TypeReference { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
