macro_rules! deps {
    () => {
        ProofTreeBuilder!();
        SearchGraph!();
        SolverDelegate!();
    };
}

macro_rules! evaluate_root_goal_for_proof_tree_raw_provider {
    () => {
        deps!();
        # [doc = " Do not call this directly, use the `tcx` query instead."] pub fn evaluate_root_goal_for_proof_tree_raw_provider < D : SolverDelegate < Interner = I > , I : Interner , > (cx : I , canonical_goal : CanonicalInput < I > ,) -> (QueryResult < I > , I :: Probe) { let mut inspect = inspect :: ProofTreeBuilder :: new () ; let canonical_result = SearchGraph :: < D > :: evaluate_root_goal_for_proof_tree (cx , cx . recursion_limit () , canonical_goal , & mut inspect ,) ; let final_revision = inspect . unwrap () ; (canonical_result , cx . mk_probe (final_revision)) }
    };
}

evaluate_root_goal_for_proof_tree_raw_provider!()