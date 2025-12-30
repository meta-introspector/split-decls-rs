// Generated macro for is_method_call (function)
macro_rules! Depcrate_expris_method_call {
() => {
// Module: crate::expr
// Provides: {"is_method_call"}
// Dependencies: {}
pub (crate) fn is_method_call (expr : & ast :: Expr) -> bool { match expr . kind { ast :: ExprKind :: MethodCall (..) => true , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) => is_method_call (expr) , _ => false , } }
};
}
