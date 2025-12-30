// Generated macro for impl_189 (impl)
macro_rules! Depcrate_traverseimpl_189 {
() => {
// Module: crate::traverse
// Provides: {"impl_189"}
// Dependencies: {}
impl TraverseType for syn :: PathArguments { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { match self { Self :: None => { } Self :: AngleBracketed (x) => x . traverse_type (f) , Self :: Parenthesized (x) => x . traverse_type (f) , } } }
};
}
