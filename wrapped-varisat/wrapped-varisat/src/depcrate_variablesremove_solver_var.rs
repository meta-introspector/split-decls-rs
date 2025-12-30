// Generated macro for remove_solver_var (function)
macro_rules! Depcrate_variablesremove_solver_var {
() => {
// Module: crate::variables
// Provides: {"remove_solver_var"}
// Dependencies: {}
# [doc = " Remove a solver var."] # [doc = ""] # [doc = " If the variable is isolated and hidden, the global variable is also removed."] pub fn remove_solver_var < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , mut VariablesP , mut VsidsP) , solver : Var ,) { decision :: remove_var (ctx . borrow () , solver) ; let variables = ctx . part_mut (VariablesP) ; let global = variables . global_from_solver_mut () . remove (solver) . expect ("no existing global var for solver var") ; variables . solver_freelist . insert (solver) ; proof :: add_step (ctx . borrow () , false , & ProofStep :: SolverVarName { global , solver : None , } ,) ; delete_global_if_unused (ctx . borrow () , global) ; }
};
}
