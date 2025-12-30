// Generated macro for impl_187 (impl)
macro_rules! Depcrate_traverseimpl_187 {
() => {
// Module: crate::traverse
// Provides: {"impl_187"}
// Dependencies: {}
impl TraverseType for syn :: TypePath { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { if let Some (qself) = self . qself . as_ref () { qself . traverse_type (f) ; } if let Some (last) = self . path . segments . last () { f (& last . ident) ; last . arguments . traverse_type (f) ; } } }
};
}
