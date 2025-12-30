// Generated macro for expr_requires_semi_to_be_stmt (function)
macro_rules! Depcrate_closuresexpr_requires_semi_to_be_stmt {
() => {
// Module: crate::closures
// Provides: {"expr_requires_semi_to_be_stmt"}
// Dependencies: {}
# [doc = " Does this expression require a semicolon to be treated"] # [doc = " as a statement? The negation of this: 'can this expression"] # [doc = " be used as a statement without a semicolon' -- is used"] # [doc = " as an early-bail-out in the parser so that, for instance,"] # [doc = "     if true {...} else {...}"] # [doc = "      |x| 5"] # [doc = " isn't parsed as (if true {...} else {...} | x) | 5"] fn expr_requires_semi_to_be_stmt (e : & ast :: Expr) -> bool { match e . kind { ast :: ExprKind :: If (..) | ast :: ExprKind :: Match (..) | ast :: ExprKind :: Block (..) | ast :: ExprKind :: While (..) | ast :: ExprKind :: Loop (..) | ast :: ExprKind :: ForLoop { .. } | ast :: ExprKind :: TryBlock (..) => false , _ => true , } }
};
}
