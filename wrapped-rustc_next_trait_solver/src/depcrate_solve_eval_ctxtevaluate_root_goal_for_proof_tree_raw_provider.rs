// Generated macro for evaluate_root_goal_for_proof_tree_raw_provider (function)
macro_rules! Depcrate_solve_eval_ctxtevaluate_root_goal_for_proof_tree_raw_provider {
() => {
// Module: crate::solve::eval_ctxt
// Provides: {"evaluate_root_goal_for_proof_tree_raw_provider"}
// Dependencies: {}
# [doc = " Do not call this directly, use the `tcx` query instead."] pub fn evaluate_root_goal_for_proof_tree_raw_provider < D : SolverDelegate < Interner = I > , I : Interner , > (cx : I , canonical_goal : CanonicalInput < I > ,) -> (QueryResult < I > , I :: Probe) { let mut inspect = inspect :: ProofTreeBuilder :: new () ; let canonical_result = SearchGraph :: < D > :: evaluate_root_goal_for_proof_tree (cx , cx . recursion_limit () , canonical_goal , & mut inspect ,) ; let final_revision = inspect . unwrap () ; (canonical_result , cx . mk_probe (final_revision)) }
};
}
