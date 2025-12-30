// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " Central solver data structure."] # [doc = ""] # [doc = " This struct contains all data kept by the solver. Most functions operating on multiple fields of"] # [doc = " the context use partial references provided by the `partial_ref` crate. This documents the data"] # [doc = " dependencies and makes the borrow checker happy without the overhead of passing individual"] # [doc = " references."] # [derive (PartialRefTarget , Default)] pub struct Context < 'a > { # [part (AnalyzeConflictP)] pub analyze_conflict : AnalyzeConflict , # [part (AssignmentP)] pub assignment : Assignment , # [part (BinaryClausesP)] pub binary_clauses : BinaryClauses , # [part (ClauseActivityP)] pub clause_activity : ClauseActivity , # [part (ClauseAllocP)] pub clause_alloc : ClauseAlloc , # [part (ClauseDbP)] pub clause_db : ClauseDb , # [part (ImplGraphP)] pub impl_graph : ImplGraph , # [part (AssumptionsP)] pub assumptions : Assumptions , # [part (ModelP)] pub model : Model , # [part (ProofP <'a >)] pub proof : Proof < 'a > , # [part (ScheduleP)] pub schedule : Schedule , # [part (SolverConfigP)] pub solver_config : SolverConfig , # [part (SolverStateP)] pub solver_state : SolverState , # [part (TmpDataP)] pub tmp_data : TmpData , # [part (TmpFlagsP)] pub tmp_flags : TmpFlags , # [part (TrailP)] pub trail : Trail , # [part (VariablesP)] pub variables : Variables , # [part (VsidsP)] pub vsids : Vsids , # [part (WatchlistsP)] pub watchlists : Watchlists , }
};
}
