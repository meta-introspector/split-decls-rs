// Generated macro for GoalEvaluation (struct)
macro_rules! Depcrate_solve_inspectGoalEvaluation {
() => {
// Module: crate::solve::inspect
// Provides: {"GoalEvaluation"}
// Dependencies: {}
# [doc = " When evaluating a goal we also store the original values"] # [doc = " for the `CanonicalVarValues` of the canonicalized goal."] # [doc = " We use this to map any [CanonicalState] from the local `InferCtxt`"] # [doc = " of the solver query to the `InferCtxt` of the caller."] # [derive_where (PartialEq , Eq , Hash ; I : Interner)] pub struct GoalEvaluation < I : Interner > { pub uncanonicalized_goal : Goal < I , I :: Predicate > , pub orig_values : Vec < I :: GenericArg > , pub final_revision : I :: Probe , pub result : QueryResult < I > , }
};
}
