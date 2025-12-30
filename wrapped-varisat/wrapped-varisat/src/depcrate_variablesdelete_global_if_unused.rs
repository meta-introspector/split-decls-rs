// Generated macro for delete_global_if_unused (function)
macro_rules! Depcrate_variablesdelete_global_if_unused {
() => {
// Module: crate::variables
// Provides: {"delete_global_if_unused"}
// Dependencies: {}
# [doc = " Delete a global variable if it is unused"] fn delete_global_if_unused < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , mut VariablesP) , global : Var ,) { let variables = ctx . part_mut (VariablesP) ; if variables . user_from_global () . get (global) . is_some () { return ; } if variables . solver_from_global () . get (global) . is_some () { return ; } let data = & mut variables . var_data [global . index ()] ; assert ! (data . sampling_mode == SamplingMode :: Hide) ; if ! data . isolated { return ; } data . deleted = true ; proof :: add_step (ctx . borrow () , false , & ProofStep :: DeleteVar { var : global }) ; ctx . part_mut (VariablesP) . global_freelist . insert (global) ; }
};
}
