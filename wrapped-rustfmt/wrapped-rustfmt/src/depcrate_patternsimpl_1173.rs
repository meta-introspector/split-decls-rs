// Generated macro for impl_1173 (impl)
macro_rules! Depcrate_patternsimpl_1173 {
() => {
// Module: crate::patterns
// Provides: {"impl_1173"}
// Dependencies: {}
impl < 'a > Rewrite for TuplePatField < 'a > { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match * self { TuplePatField :: Pat (p) => p . rewrite_result (context , shape) , TuplePatField :: Dotdot (_) => Ok (".." . to_string ()) , } } }
};
}
