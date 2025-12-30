// Generated macro for rewrite_cond (function)
macro_rules! Depcrate_exprrewrite_cond {
() => {
// Module: crate::expr
// Provides: {"rewrite_cond"}
// Dependencies: {}
pub (crate) fn rewrite_cond (context : & RewriteContext < '_ > , expr : & ast :: Expr , shape : Shape ,) -> Option < String > { match expr . kind { ast :: ExprKind :: Match (ref cond , _ , MatchKind :: Prefix) => { let cond_shape = match context . config . indent_style () { IndentStyle :: Visual => shape . shrink_left_opt (6) . and_then (| s | s . sub_width_opt (2)) ? , IndentStyle :: Block => shape . offset_left_opt (8) ? , } ; cond . rewrite (context , cond_shape) } _ => to_control_flow (expr , ExprType :: SubExpression) . and_then (| control_flow | { let alt_block_sep = String :: from ("\n") + & shape . indent . block_only () . to_string (context . config) ; control_flow . rewrite_cond (context , shape , & alt_block_sep) . ok () . map (| rw | rw . 0) }) , } }
};
}
