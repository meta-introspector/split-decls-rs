// Generated macro for left_most_sub_expr (function)
macro_rules! Depcrate_utilsleft_most_sub_expr {
() => {
// Module: crate::utils
// Provides: {"left_most_sub_expr"}
// Dependencies: {}
# [inline] pub (crate) fn left_most_sub_expr (e : & ast :: Expr) -> & ast :: Expr { match e . kind { ast :: ExprKind :: Call (ref e , _) | ast :: ExprKind :: Binary (_ , ref e , _) | ast :: ExprKind :: Cast (ref e , _) | ast :: ExprKind :: Type (ref e , _) | ast :: ExprKind :: Assign (ref e , _ , _) | ast :: ExprKind :: AssignOp (_ , ref e , _) | ast :: ExprKind :: Field (ref e , _) | ast :: ExprKind :: Index (ref e , _ , _) | ast :: ExprKind :: Range (Some (ref e) , _ , _) | ast :: ExprKind :: Try (ref e) => left_most_sub_expr (e) , _ => e , } }
};
}
