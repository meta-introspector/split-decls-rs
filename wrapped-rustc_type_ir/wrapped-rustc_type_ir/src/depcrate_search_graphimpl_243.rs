// Generated macro for impl_243 (impl)
macro_rules! Depcrate_search_graphimpl_243 {
() => {
// Module: crate::search_graph
// Provides: {"impl_243"}
// Dependencies: {}
impl < X : Cx > EvaluationResult < X > { fn finalize (final_entry : StackEntry < X > , encountered_overflow : bool , result : X :: Result ,) -> EvaluationResult < X > { EvaluationResult { encountered_overflow , required_depth : final_entry . required_depth , heads : final_entry . heads , nested_goals : final_entry . nested_goals , result , } } }
};
}
