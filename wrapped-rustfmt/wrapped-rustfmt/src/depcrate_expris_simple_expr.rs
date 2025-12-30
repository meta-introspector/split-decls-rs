// Generated macro for is_simple_expr (function)
macro_rules! Depcrate_expris_simple_expr {
() => {
// Module: crate::expr
// Provides: {"is_simple_expr"}
// Dependencies: {}
pub (crate) fn is_simple_expr (expr : & ast :: Expr) -> bool { match expr . kind { ast :: ExprKind :: Lit (..) => true , ast :: ExprKind :: Path (ref qself , ref path) => qself . is_none () && path . segments . len () <= 1 , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) | ast :: ExprKind :: Field (ref expr , _) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) => is_simple_expr (expr) , ast :: ExprKind :: Index (ref lhs , ref rhs , _) => is_simple_expr (lhs) && is_simple_expr (rhs) , ast :: ExprKind :: Repeat (ref lhs , ref rhs) => { is_simple_expr (lhs) && is_simple_expr (& * rhs . value) } _ => false , } }
};
}
