// Generated macro for is_every_expr_simple (function)
macro_rules! Depcrate_expris_every_expr_simple {
() => {
// Module: crate::expr
// Provides: {"is_every_expr_simple"}
// Dependencies: {}
pub (crate) fn is_every_expr_simple (lists : & [OverflowableItem < '_ >]) -> bool { lists . iter () . all (OverflowableItem :: is_simple) }
};
}
