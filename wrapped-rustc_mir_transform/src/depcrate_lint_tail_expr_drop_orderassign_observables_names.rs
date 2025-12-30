// Generated macro for assign_observables_names (function)
macro_rules! Depcrate_lint_tail_expr_drop_orderassign_observables_names {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"assign_observables_names"}
// Dependencies: {}
# [doc = " Assign names for anonymous or temporary values for diagnosis"] fn assign_observables_names (locals : impl IntoIterator < Item = Local > , user_names : & FxIndexMap < Local , Symbol > ,) -> FxIndexMap < Local , (String , bool) > { let mut names = FxIndexMap :: default () ; let mut assigned_names = FxHashSet :: default () ; let mut idx = 0u64 ; let mut fresh_name = | | { idx += 1 ; (format ! ("#{idx}") , true) } ; for local in locals { let name = if let Some (name) = user_names . get (& local) { let name = name . as_str () ; if assigned_names . contains (name) { fresh_name () } else { (name . to_owned () , false) } } else { fresh_name () } ; assigned_names . insert (name . 0 . clone ()) ; names . insert (local , name) ; } names }
};
}
