// Generated macro for impl_1340 (impl)
macro_rules! Depcrate_unusedimpl_1340 {
() => {
// Module: crate::unused
// Provides: {"impl_1340"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for PathStatements { fn check_stmt (& mut self , cx : & LateContext < '_ > , s : & hir :: Stmt < '_ >) { if let hir :: StmtKind :: Semi (expr) = s . kind && let hir :: ExprKind :: Path (_) = expr . kind { let ty = cx . typeck_results () . expr_ty (expr) ; if ty . needs_drop (cx . tcx , cx . typing_env ()) { let sub = if let Ok (snippet) = cx . sess () . source_map () . span_to_snippet (expr . span) { PathStatementDropSub :: Suggestion { span : s . span , snippet } } else { PathStatementDropSub :: Help { span : s . span } } ; cx . emit_span_lint (PATH_STATEMENTS , s . span , PathStatementDrop { sub }) } else { cx . emit_span_lint (PATH_STATEMENTS , s . span , PathStatementNoEffect) ; } } } }
};
}
