// Generated macro for solver_from_user_lits (function)
macro_rules! Depcrate_variablessolver_from_user_lits {
() => {
// Module: crate::variables
// Provides: {"solver_from_user_lits"}
// Dependencies: {}
# [doc = " Maps a slice of user lits to solver lits using [`solver_from_user`]."] pub fn solver_from_user_lits < 'a > (mut ctx : partial ! (Context <'a >, mut AnalyzeConflictP , mut AssignmentP , mut BinaryClausesP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut VariablesP , mut VsidsP , mut WatchlistsP ,) , solver_lits : & mut Vec < Lit > , user_lits : & [Lit] , require_sampling : bool ,) { solver_lits . clear () ; solver_lits . extend (user_lits . iter () . map (| user_lit | { user_lit . map_var (| user_var | solver_from_user (ctx . borrow () , user_var , require_sampling)) })) }
};
}
