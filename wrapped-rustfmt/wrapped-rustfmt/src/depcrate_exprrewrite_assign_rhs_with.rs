// Generated macro for rewrite_assign_rhs_with (function)
macro_rules! Depcrate_exprrewrite_assign_rhs_with {
() => {
// Module: crate::expr
// Provides: {"rewrite_assign_rhs_with"}
// Dependencies: {}
pub (crate) fn rewrite_assign_rhs_with < S : Into < String > , R : Rewrite > (context : & RewriteContext < '_ > , lhs : S , ex : & R , shape : Shape , rhs_kind : & RhsAssignKind < '_ > , rhs_tactics : RhsTactics ,) -> RewriteResult { let lhs = lhs . into () ; let rhs = rewrite_assign_rhs_expr (context , & lhs , ex , shape , rhs_kind , rhs_tactics) ? ; Ok (lhs + & rhs) }
};
}
