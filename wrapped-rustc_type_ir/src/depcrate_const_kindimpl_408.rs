// Generated macro for impl_408 (impl)
macro_rules! Depcrate_const_kindimpl_408 {
() => {
// Module: crate::const_kind
// Provides: {"impl_408"}
// Dependencies: {}
impl fmt :: Debug for InferConst { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { InferConst :: Var (var) => write ! (f , "{var:?}") , InferConst :: Fresh (var) => write ! (f , "Fresh({var:?})") , } } }
};
}
