// Generated macro for value_assigned_to_local (function)
macro_rules! Depcrate_rustc_peekvalue_assigned_to_local {
() => {
// Module: crate::rustc_peek
// Provides: {"value_assigned_to_local"}
// Dependencies: {}
# [doc = " If `stmt` is an assignment where the LHS is the given local (with no projections), returns the"] # [doc = " RHS of the assignment."] fn value_assigned_to_local < 'a , 'tcx > (stmt : & 'a mir :: Statement < 'tcx > , local : Local ,) -> Option < & 'a mir :: Rvalue < 'tcx > > { if let mir :: StatementKind :: Assign (box (place , rvalue)) = & stmt . kind && let Some (l) = place . as_local () && local == l { return Some (& * rvalue) ; } None }
};
}
