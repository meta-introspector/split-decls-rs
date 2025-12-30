// Generated macro for solver_from_global (function)
macro_rules! Depcrate_variablessolver_from_global {
() => {
// Module: crate::variables
// Provides: {"solver_from_global"}
// Dependencies: {}
# [doc = " Maps an existing global variable to a solver variable."] # [doc = ""] # [doc = " If no matching solver variable exists a new one is allocated."] pub fn solver_from_global < 'a > (mut ctx : partial ! (Context <'a >, mut AnalyzeConflictP , mut AssignmentP , mut BinaryClausesP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut VariablesP , mut VsidsP , mut WatchlistsP ,) , global : Var ,) -> Var { let variables = ctx . part_mut (VariablesP) ; debug_assert ! (! variables . var_data [global . index ()] . deleted) ; match variables . solver_from_global () . get (global) { Some (solver) => solver , None => { let solver = variables . next_unmapped_solver () ; let old_watermark = variables . global_from_solver () . watermark () ; variables . solver_from_global_mut () . insert (solver , global) ; variables . solver_freelist . remove (& solver) ; let new_watermark = variables . global_from_solver () . watermark () ; if new_watermark > old_watermark { set_var_count (ctx . borrow () , new_watermark) ; } initialize_solver_var (ctx . borrow () , solver , global) ; proof :: add_step (ctx . borrow () , false , & ProofStep :: SolverVarName { global , solver : Some (solver) , } ,) ; solver } } }
};
}
