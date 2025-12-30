// Generated macro for EvaluationResult (struct)
macro_rules! Depcrate_search_graphEvaluationResult {
() => {
// Module: crate::search_graph
// Provides: {"EvaluationResult"}
// Dependencies: {}
# [doc = " The final result of evaluating a goal."] # [doc = ""] # [doc = " We reset `encountered_overflow` when reevaluating a goal,"] # [doc = " but need to track whether we've hit the recursion limit at"] # [doc = " all for correctness."] # [doc = ""] # [doc = " We've previously simply returned the final `StackEntry` but this"] # [doc = " made it easy to accidentally drop information from the previous"] # [doc = " evaluation."] # [derive_where (Debug ; X : Cx)] struct EvaluationResult < X : Cx > { encountered_overflow : bool , required_depth : usize , heads : CycleHeads , nested_goals : NestedGoals < X > , result : X :: Result , }
};
}
