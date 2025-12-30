// Generated macro for impl_196 (impl)
macro_rules! Depcrate_traverseimpl_196 {
() => {
// Module: crate::traverse
// Provides: {"impl_196"}
// Dependencies: {}
impl TraverseType for syn :: TypeTuple { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { for ty in self . elems . iter () { ty . traverse_type (f) ; } } }
};
}
