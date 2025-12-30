// Generated macro for impl_181 (impl)
macro_rules! Depcrate_traverseimpl_181 {
() => {
// Module: crate::traverse
// Provides: {"impl_181"}
// Dependencies: {}
impl TraverseType for Type { fn traverse_type < F > (& self , f : & mut F) where F : FnMut (& Ident) , { match self { Type :: Array (x) => x . traverse_type (f) , Type :: BareFn (x) => x . traverse_type (f) , Type :: Group (x) => x . traverse_type (f) , Type :: Paren (x) => x . traverse_type (f) , Type :: Path (x) => x . traverse_type (f) , Type :: Ptr (x) => x . traverse_type (f) , Type :: Reference (x) => x . traverse_type (f) , Type :: Slice (x) => x . traverse_type (f) , Type :: Tuple (x) => x . traverse_type (f) , _ => { } } } }
};
}
