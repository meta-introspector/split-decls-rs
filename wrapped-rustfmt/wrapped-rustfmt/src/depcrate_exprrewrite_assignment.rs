// Generated macro for rewrite_assignment (function)
macro_rules! Depcrate_exprrewrite_assignment {
() => {
// Module: crate::expr
// Provides: {"rewrite_assignment"}
// Dependencies: {}
fn rewrite_assignment (context : & RewriteContext < '_ > , lhs : & ast :: Expr , rhs : & ast :: Expr , op : Option < & ast :: BinOp > , shape : Shape ,) -> RewriteResult { let operator_str = match op { Some (op) => context . snippet (op . span) , None => "=" , } ; let lhs_shape = shape . sub_width (operator_str . len () + 1 , lhs . span ()) ? ; let lhs_str = format ! ("{} {}" , lhs . rewrite_result (context , lhs_shape) ?, operator_str) ; rewrite_assign_rhs (context , lhs_str , rhs , & RhsAssignKind :: Expr (& rhs . kind , rhs . span) , shape ,) }
};
}
