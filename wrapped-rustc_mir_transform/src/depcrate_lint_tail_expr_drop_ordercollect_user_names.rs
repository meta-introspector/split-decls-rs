// Generated macro for collect_user_names (function)
macro_rules! Depcrate_lint_tail_expr_drop_ordercollect_user_names {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"collect_user_names"}
// Dependencies: {}
# [doc = " Extract binding names if available for diagnosis"] fn collect_user_names (body : & Body < '_ >) -> FxIndexMap < Local , Symbol > { let mut names = FxIndexMap :: default () ; for var_debug_info in & body . var_debug_info { if let mir :: VarDebugInfoContents :: Place (place) = & var_debug_info . value && let Some (local) = place . local_or_deref_local () { names . entry (local) . or_insert (var_debug_info . name) ; } } names }
};
}
