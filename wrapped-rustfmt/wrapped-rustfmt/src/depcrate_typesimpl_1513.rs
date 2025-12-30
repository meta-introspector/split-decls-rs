// Generated macro for impl_1513 (impl)
macro_rules! Depcrate_typesimpl_1513 {
() => {
// Module: crate::types
// Provides: {"impl_1513"}
// Dependencies: {}
impl Rewrite for ast :: AssocItemConstraintKind { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match self { ast :: AssocItemConstraintKind :: Equality { term } => match term { Term :: Ty (ty) => ty . rewrite_result (context , shape) , Term :: Const (c) => c . rewrite_result (context , shape) , } , ast :: AssocItemConstraintKind :: Bound { bounds } => bounds . rewrite_result (context , shape) , } } }
};
}
