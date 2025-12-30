// Generated macro for remove_user_mapping (function)
macro_rules! Depcrate_variablesremove_user_mapping {
() => {
// Module: crate::variables
// Provides: {"remove_user_mapping"}
// Dependencies: {}
# [doc = " Remove a user/global var mapping."] pub fn remove_user_mapping < 'a > (mut ctx : partial ! (Context <'a >, mut ClausesP , mut ProcessingP <'a >, mut VariablesP , CheckerStateP) , global_var : Var ,) -> Result < () , CheckerError > { ensure_var (ctx . borrow () , global_var) ; let variables = ctx . part_mut (VariablesP) ; let var_data = & variables . var_data [global_var . index ()] ; if var_data . user_var . is_some () { process_step (ctx . borrow () , & CheckedProofStep :: UserVar { var : global_var , user_var : None , } ,) ? ; } else { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("no user name to remove for variable {:?}" , global_var) ,)) ; } let variables = ctx . part_mut (VariablesP) ; let var_data = & mut variables . var_data [global_var . index ()] ; if let Some (user_var) = var_data . user_var { variables . used_user_vars . remove (& user_var) ; var_data . user_var = None ; var_data . sampling_mode = SamplingMode :: Hide ; } Ok (()) }
};
}
