// Generated macro for rewrite_assign_rhs (function)
macro_rules! Depcrate_exprrewrite_assign_rhs {
() => {
// Module: crate::expr
// Provides: {"rewrite_assign_rhs"}
// Dependencies: {}
pub (crate) fn rewrite_assign_rhs < S : Into < String > , R : Rewrite > (context : & RewriteContext < '_ > , lhs : S , ex : & R , rhs_kind : & RhsAssignKind < '_ > , shape : Shape ,) -> RewriteResult { rewrite_assign_rhs_with (context , lhs , ex , shape , rhs_kind , RhsTactics :: Default) }
};
}
