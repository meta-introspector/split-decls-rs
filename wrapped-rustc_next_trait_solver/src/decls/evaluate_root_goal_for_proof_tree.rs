macro_rules! deps {
    () => {
        SolverDelegate!();
        GoalEvaluation!();
        EvalCtxt!();
    };
}

macro_rules! evaluate_root_goal_for_proof_tree {
    () => {
        deps!();
        # [doc = " Evaluate a goal to build a proof tree."] # [doc = ""] # [doc = " This is a copy of [EvalCtxt::evaluate_goal_raw] which avoids relying on the"] # [doc = " [EvalCtxt] and uses a separate cache."] pub (super) fn evaluate_root_goal_for_proof_tree < D : SolverDelegate < Interner = I > , I : Interner > (delegate : & D , goal : Goal < I , I :: Predicate > , origin_span : I :: Span ,) -> (Result < NestedNormalizationGoals < I > , NoSolution > , inspect :: GoalEvaluation < I >) { let opaque_types = delegate . clone_opaque_types_lookup_table () ; let (goal , opaque_types) = eager_resolve_vars (delegate , (goal , opaque_types)) ; let (orig_values , canonical_goal) = EvalCtxt :: canonicalize_goal (delegate , goal , opaque_types) ; let (canonical_result , final_revision) = delegate . cx () . evaluate_root_goal_for_proof_tree_raw (canonical_goal) ; let proof_tree = inspect :: GoalEvaluation { uncanonicalized_goal : goal , orig_values , final_revision , result : canonical_result , } ; let response = match canonical_result { Err (e) => return (Err (e) , proof_tree) , Ok (response) => response , } ; let (normalization_nested_goals , _certainty) = EvalCtxt :: instantiate_and_apply_query_response (delegate , goal . param_env , & proof_tree . orig_values , response , origin_span ,) ; (Ok (normalization_nested_goals) , proof_tree) }
    };
}

evaluate_root_goal_for_proof_tree!()