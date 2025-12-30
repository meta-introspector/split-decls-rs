// Generated macro for impl_1530 (impl)
macro_rules! Depcrate_typesimpl_1530 {
() => {
// Module: crate::types
// Provides: {"impl_1530"}
// Dependencies: {}
impl Rewrite for ast :: TyPat { fn rewrite (& self , context : & RewriteContext < '_ > , shape : Shape) -> Option < String > { self . rewrite_result (context , shape) . ok () } fn rewrite_result (& self , context : & RewriteContext < '_ > , shape : Shape) -> RewriteResult { match self . kind { ast :: TyPatKind :: Range (ref lhs , ref rhs , ref end_kind) => { rewrite_range_pat (context , shape , lhs , rhs , end_kind , self . span) } ast :: TyPatKind :: Or (ref variants) => { let mut first = true ; let mut s = String :: new () ; for variant in variants { if first { first = false } else { s . push_str (" | ") ; } s . push_str (& variant . rewrite_result (context , shape) ?) ; } Ok (s) } ast :: TyPatKind :: Err (_) => Err (RewriteError :: Unknown) , } } }
};
}
