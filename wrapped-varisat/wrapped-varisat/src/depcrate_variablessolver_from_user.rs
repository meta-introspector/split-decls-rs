// Generated macro for solver_from_user (function)
macro_rules! Depcrate_variablessolver_from_user {
() => {
// Module: crate::variables
// Provides: {"solver_from_user"}
// Dependencies: {}
# [doc = " Maps a user variable to a solver variable."] # [doc = ""] # [doc = " Allocates global and solver variables as requried."] pub fn solver_from_user < 'a > (mut ctx : partial ! (Context <'a >, mut AnalyzeConflictP , mut AssignmentP , mut BinaryClausesP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut VariablesP , mut VsidsP , mut WatchlistsP ,) , user : Var , require_sampling : bool ,) -> Var { let global = global_from_user (ctx . borrow () , user , require_sampling) ; solver_from_global (ctx . borrow () , global) }
};
}
