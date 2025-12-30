// Generated macro for is_nested_call (function)
macro_rules! Depcrate_expris_nested_call {
() => {
// Module: crate::expr
// Provides: {"is_nested_call"}
// Dependencies: {}
pub (crate) fn is_nested_call (expr : & ast :: Expr) -> bool { match expr . kind { ast :: ExprKind :: Call (..) | ast :: ExprKind :: MacCall (..) => true , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) => is_nested_call (expr) , _ => false , } }
};
}
