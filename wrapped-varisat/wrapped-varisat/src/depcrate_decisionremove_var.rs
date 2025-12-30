// Generated macro for remove_var (function)
macro_rules! Depcrate_decisionremove_var {
() => {
// Module: crate::decision
// Provides: {"remove_var"}
// Dependencies: {}
# [doc = " Remove a variable from the decision heuristics."] pub fn remove_var (mut ctx : partial ! (Context , mut VsidsP) , var : Var) { ctx . part_mut (VsidsP) . make_unavailable (var) ; }
};
}
