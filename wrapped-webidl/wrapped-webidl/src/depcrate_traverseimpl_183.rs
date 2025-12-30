// Generated macro for impl_183 (impl)
macro_rules! Depcrate_traverseimpl_183 {
() => {
// Module: crate::traverse
// Provides: {"impl_183"}
// Dependencies: {}
impl TraverseType for syn :: TypeBareFn { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { for input in self . inputs . iter () { input . ty . traverse_type (f) ; } self . output . traverse_type (f) ; } }
};
}
