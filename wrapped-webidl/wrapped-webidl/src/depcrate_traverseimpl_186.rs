// Generated macro for impl_186 (impl)
macro_rules! Depcrate_traverseimpl_186 {
() => {
// Module: crate::traverse
// Provides: {"impl_186"}
// Dependencies: {}
impl TraverseType for syn :: TypeParen { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { self . elem . traverse_type (f) ; } }
};
}
