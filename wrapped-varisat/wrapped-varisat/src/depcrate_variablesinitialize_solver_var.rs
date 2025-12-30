// Generated macro for initialize_solver_var (function)
macro_rules! Depcrate_variablesinitialize_solver_var {
() => {
// Module: crate::variables
// Provides: {"initialize_solver_var"}
// Dependencies: {}
# [doc = " Initialize a newly allocated solver variable"] pub fn initialize_solver_var (mut ctx : partial ! (Context , mut AssignmentP , mut ImplGraphP , mut VsidsP , VariablesP) , solver : Var , global : Var ,) { let (variables , mut ctx) = ctx . split_part (VariablesP) ; let data = & variables . var_data [global . index ()] ; ctx . part_mut (AssignmentP) . set_var (solver , data . unit) ; if data . unit . is_some () { ctx . part_mut (ImplGraphP) . update_removed_unit (solver) ; } decision :: initialize_var (ctx . borrow () , solver , data . unit . is_none ()) ; }
};
}
