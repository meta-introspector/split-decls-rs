// Generated macro for set_assumptions (function)
macro_rules! Depcrate_assumptionsset_assumptions {
() => {
// Module: crate::assumptions
// Provides: {"set_assumptions"}
// Dependencies: {}
# [doc = " Change the currently active assumptions."] # [doc = ""] # [doc = " The input uses user variable names."] pub fn set_assumptions < 'a > (mut ctx : partial ! (Context <'a >, mut AnalyzeConflictP , mut AssignmentP , mut AssumptionsP , mut BinaryClausesP , mut ImplGraphP , mut ProofP <'a >, mut SolverStateP , mut TmpFlagsP , mut TrailP , mut VariablesP , mut VsidsP , mut WatchlistsP ,) , user_assumptions : & [Lit] ,) { full_restart (ctx . borrow ()) ; let state = ctx . part_mut (SolverStateP) ; state . sat_state = match state . sat_state { SatState :: Unsat => SatState :: Unsat , SatState :: Sat | SatState :: UnsatUnderAssumptions | SatState :: Unknown => SatState :: Unknown , } ; let (assumptions , mut ctx_2) = ctx . split_part_mut (AssumptionsP) ; for lit in assumptions . assumptions . iter () { ctx_2 . part_mut (VariablesP) . var_data_solver_mut (lit . var ()) . assumed = false ; } variables :: solver_from_user_lits (ctx_2 . borrow () , & mut assumptions . assumptions , user_assumptions , true ,) ; for lit in assumptions . assumptions . iter () { ctx_2 . part_mut (VariablesP) . var_data_solver_mut (lit . var ()) . assumed = true ; } proof :: add_step (ctx_2 . borrow () , true , & ProofStep :: Assumptions { assumptions : & assumptions . assumptions , } ,) ; }
};
}
