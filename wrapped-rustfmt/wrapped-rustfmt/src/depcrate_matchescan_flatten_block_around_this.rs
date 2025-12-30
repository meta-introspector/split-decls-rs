// Generated macro for can_flatten_block_around_this (function)
macro_rules! Depcrate_matchescan_flatten_block_around_this {
() => {
// Module: crate::matches
// Provides: {"can_flatten_block_around_this"}
// Dependencies: {}
fn can_flatten_block_around_this (body : & ast :: Expr) -> bool { match body . kind { ast :: ExprKind :: If (..) => false , ast :: ExprKind :: ForLoop { .. } | ast :: ExprKind :: While (..) => false , ast :: ExprKind :: Loop (..) | ast :: ExprKind :: Match (..) | ast :: ExprKind :: Block (..) | ast :: ExprKind :: Closure (..) | ast :: ExprKind :: Array (..) | ast :: ExprKind :: Call (..) | ast :: ExprKind :: MethodCall (..) | ast :: ExprKind :: MacCall (..) | ast :: ExprKind :: Struct (..) | ast :: ExprKind :: Tup (..) => true , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) | ast :: ExprKind :: Index (ref expr , _ , _) | ast :: ExprKind :: Cast (ref expr , _) => can_flatten_block_around_this (expr) , _ => false , } }
};
}
