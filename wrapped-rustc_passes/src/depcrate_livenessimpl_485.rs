// Generated macro for impl_485 (impl)
macro_rules! Depcrate_livenessimpl_485 {
() => {
// Module: crate::liveness
// Provides: {"impl_485"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for CollectLitsVisitor < 'tcx > { fn visit_expr (& mut self , expr : & 'tcx Expr < 'tcx >) { if let hir :: ExprKind :: Lit (_) = expr . kind { self . lit_exprs . push (expr) ; } intravisit :: walk_expr (self , expr) ; } }
};
}
