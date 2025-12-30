// Generated macro for SearchGraphDelegate (struct)
macro_rules! Depcrate_solve_search_graphSearchGraphDelegate {
() => {
// Module: crate::solve::search_graph
// Provides: {"SearchGraphDelegate"}
// Dependencies: {}
# [doc = " This type is never constructed. We only use it to implement `search_graph::Delegate`"] # [doc = " for all types which impl `SolverDelegate` and doing it directly fails in coherence."] pub (super) struct SearchGraphDelegate < D : SolverDelegate > { _marker : PhantomData < D > , }
};
}
