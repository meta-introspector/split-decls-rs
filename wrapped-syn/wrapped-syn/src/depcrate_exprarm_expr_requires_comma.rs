// Generated macro for arm_expr_requires_comma (function)
macro_rules! Depcrate_exprarm_expr_requires_comma {
() => {
// Module: crate::expr
// Provides: {"arm_expr_requires_comma"}
// Dependencies: {}
# [cfg (any (feature = "parsing" , feature = "printing"))] # [cfg (feature = "full")] fn arm_expr_requires_comma (expr : & Expr) -> bool { match expr . node { ExprKind :: Block (..) | ExprKind :: If (..) | ExprKind :: IfLet (..) | ExprKind :: Match (..) | ExprKind :: While (..) | ExprKind :: WhileLet (..) | ExprKind :: Loop (..) | ExprKind :: ForLoop (..) | ExprKind :: Catch (..) => false , _ => true , } }
};
}
