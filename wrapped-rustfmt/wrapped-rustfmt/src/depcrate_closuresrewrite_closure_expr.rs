// Generated macro for rewrite_closure_expr (function)
macro_rules! Depcrate_closuresrewrite_closure_expr {
() => {
// Module: crate::closures
// Provides: {"rewrite_closure_expr"}
// Dependencies: {}
fn rewrite_closure_expr (expr : & ast :: Expr , prefix : & str , context : & RewriteContext < '_ > , shape : Shape ,) -> RewriteResult { fn allow_multi_line (expr : & ast :: Expr) -> bool { match expr . kind { ast :: ExprKind :: Match (..) | ast :: ExprKind :: Gen (..) | ast :: ExprKind :: Block (..) | ast :: ExprKind :: TryBlock (..) | ast :: ExprKind :: Loop (..) | ast :: ExprKind :: Struct (..) => true , ast :: ExprKind :: AddrOf (_ , _ , ref expr) | ast :: ExprKind :: Try (ref expr) | ast :: ExprKind :: Unary (_ , ref expr) | ast :: ExprKind :: Cast (ref expr , _) => allow_multi_line (expr) , _ => false , } } let veto_multiline = (! allow_multi_line (expr) && ! context . inside_macro ()) || context . config . force_multiline_blocks () ; expr . rewrite_result (context , shape) . and_then (| rw | { if veto_multiline && rw . contains ('\n') { Err (RewriteError :: Unknown) } else { Ok (rw) } }) . map (| rw | format ! ("{} {}" , prefix , rw)) }
};
}
