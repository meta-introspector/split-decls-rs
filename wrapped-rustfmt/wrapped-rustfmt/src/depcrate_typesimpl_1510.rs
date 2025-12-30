// Generated macro for impl_1510 (impl)
macro_rules! Depcrate_typesimpl_1510 {
() => {
// Module: crate::types
// Provides: {"impl_1510"}
// Dependencies: {}
impl < 'a > Rewrite for SegmentParam < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { SegmentParam :: Const (const_) => const_ . rewrite_result (context , shape) , SegmentParam :: LifeTime (lt) => lt . rewrite_result (context , shape) , SegmentParam :: Type (ty) => ty . rewrite_result (context , shape) , SegmentParam :: Binding (atc) => atc . rewrite_result (context , shape) , } } }
};
}
