// Generated macro for place_has_common_prefix (function)
macro_rules! Depcrate_lint_tail_expr_drop_orderplace_has_common_prefix {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"place_has_common_prefix"}
// Dependencies: {}
fn place_has_common_prefix < 'tcx > (left : & Place < 'tcx > , right : & Place < 'tcx >) -> bool { left . local == right . local && left . projection . iter () . zip (right . projection) . all (| (left , right) | left == right) }
};
}
