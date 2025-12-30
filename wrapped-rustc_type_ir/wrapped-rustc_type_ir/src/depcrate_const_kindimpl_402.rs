// Generated macro for impl_402 (impl)
macro_rules! Depcrate_const_kindimpl_402 {
() => {
// Module: crate::const_kind
// Provides: {"impl_402"}
// Dependencies: {}
impl < I : Interner > fmt :: Debug for ConstKind < I > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { use ConstKind :: * ; match self { Param (param) => write ! (f , "{param:?}") , Infer (var) => write ! (f , "{var:?}") , Bound (debruijn , var) => crate :: debug_bound_var (f , * debruijn , var) , Placeholder (placeholder) => write ! (f , "{placeholder:?}") , Unevaluated (uv) => write ! (f , "{uv:?}") , Value (val) => write ! (f , "{val:?}") , Error (_) => write ! (f , "{{const error}}") , Expr (expr) => write ! (f , "{expr:?}") , } } }
};
}
