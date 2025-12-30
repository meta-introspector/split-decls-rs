// Generated macro for impl_191 (impl)
macro_rules! Depcrate_traverseimpl_191 {
() => {
// Module: crate::traverse
// Provides: {"impl_191"}
// Dependencies: {}
impl TraverseType for syn :: GenericArgument { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { match self { Self :: Type (x) => x . traverse_type (f) , Self :: AssocType (x) => x . traverse_type (f) , _ => { } } } }
};
}
