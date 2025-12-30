// Generated macro for impl_193 (impl)
macro_rules! Depcrate_traverseimpl_193 {
() => {
// Module: crate::traverse
// Provides: {"impl_193"}
// Dependencies: {}
impl TraverseType for syn :: ParenthesizedGenericArguments { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { for ty in self . inputs . iter () { ty . traverse_type (f) ; } self . output . traverse_type (f) ; } }
};
}
