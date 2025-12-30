// Generated macro for impl_275 (impl)
macro_rules! Depcrate_typekindsimpl_275 {
() => {
// Module: crate::typekinds
// Provides: {"impl_275"}
// Dependencies: {}
impl fmt :: Display for TypeKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Vector (ty) => write ! (f , "{ty}") , Self :: Pointer (ty , _) => write ! (f , "{ty}") , Self :: Base (ty) => write ! (f , "{ty}") , Self :: Wildcard (w) => write ! (f , "{{{w}}}") , Self :: Custom (s) => write ! (f , "{s}") , } } }
};
}
