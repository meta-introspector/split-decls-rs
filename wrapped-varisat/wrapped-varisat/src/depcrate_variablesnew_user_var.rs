// Generated macro for new_user_var (function)
macro_rules! Depcrate_variablesnew_user_var {
() => {
// Module: crate::variables
// Provides: {"new_user_var"}
// Dependencies: {}
# [doc = " Allocates a currently unused user variable."] # [doc = ""] # [doc = " This is either a user variable above any user variable used so far, or a user variable that was"] # [doc = " previously hidden by the user."] pub fn new_user_var < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , mut VariablesP) ,) -> Var { let variables = ctx . part_mut (VariablesP) ; let user_var = variables . next_unmapped_user () ; global_from_user (ctx . borrow () , user_var , false) ; user_var }
};
}
