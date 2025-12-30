// Generated macro for veto_block (function)
macro_rules! Depcrate_closuresveto_block {
() => {
// Module: crate::closures
// Provides: {"veto_block"}
// Dependencies: {}
fn veto_block (e : & ast :: Expr) -> bool { match e . kind { ast :: ExprKind :: Call (..) | ast :: ExprKind :: Binary (..) | ast :: ExprKind :: Cast (..) | ast :: ExprKind :: Type (..) | ast :: ExprKind :: Assign (..) | ast :: ExprKind :: AssignOp (..) | ast :: ExprKind :: Field (..) | ast :: ExprKind :: Index (..) | ast :: ExprKind :: Range (..) | ast :: ExprKind :: Try (..) => true , _ => false , } }
};
}
