// Generated macro for impl_409 (impl)
macro_rules! Depcrate_check_tail_callsimpl_409 {
() => {
// Module: crate::check_tail_calls
// Provides: {"impl_409"}
// Dependencies: {}
impl < 'a , 'tcx > Visitor < 'a , 'tcx > for TailCallCkVisitor < 'a , 'tcx > { fn thir (& self) -> & 'a Thir < 'tcx > { & self . thir } fn visit_expr (& mut self , expr : & 'a Expr < 'tcx >) { ensure_sufficient_stack (| | { if let ExprKind :: Become { value } = expr . kind { let call = & self . thir [value] ; self . check_tail_call (call , expr) ; } visit :: walk_expr (self , expr) ; }) ; } }
};
}
