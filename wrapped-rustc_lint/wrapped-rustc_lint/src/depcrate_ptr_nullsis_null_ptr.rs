// Generated macro for is_null_ptr (function)
macro_rules! Depcrate_ptr_nullsis_null_ptr {
() => {
// Module: crate::ptr_nulls
// Provides: {"is_null_ptr"}
// Dependencies: {}
# [doc = " Checks if the given expression is a null pointer (modulo casting)"] fn is_null_ptr < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) -> Option < Span > { let (expr , _) = peel_casts (cx , expr) ; if let ExprKind :: Call (path , []) = expr . kind && let ExprKind :: Path (ref qpath) = path . kind && let Some (def_id) = cx . qpath_res (qpath , path . hir_id) . opt_def_id () && let Some (diag_item) = cx . tcx . get_diagnostic_name (def_id) { (diag_item == sym :: ptr_null || diag_item == sym :: ptr_null_mut) . then_some (expr . span) } else if let ExprKind :: Lit (spanned) = expr . kind && let LitKind :: Int (v , _) = spanned . node { (v == 0) . then_some (expr . span) } else { None } }
};
}
