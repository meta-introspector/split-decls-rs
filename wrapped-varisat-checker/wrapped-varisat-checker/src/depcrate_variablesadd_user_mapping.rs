// Generated macro for add_user_mapping (function)
macro_rules! Depcrate_variablesadd_user_mapping {
() => {
// Module: crate::variables
// Provides: {"add_user_mapping"}
// Dependencies: {}
# [doc = " Add a user/global var mapping."] pub fn add_user_mapping < 'a > (mut ctx : partial ! (Context <'a >, mut ClausesP , mut ProcessingP <'a >, mut VariablesP , CheckerStateP) , global_var : Var , user_var : Var ,) -> Result < () , CheckerError > { ensure_var (ctx . borrow () , global_var) ; let mut ctx_in = ctx ; let (variables , ctx) = ctx_in . split_part_mut (VariablesP) ; if user_var . index () >= variables . var_data . len () || variables . used_user_vars . contains (& user_var) { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("user name {:?} used for two different variables" , user_var) ,)) ; } let var_data = & mut variables . var_data [global_var . index ()] ; if var_data . sampling_mode == SamplingMode :: Hide { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("user name added to variable {:?} which is still hidden" , global_var) ,)) ; } if var_data . user_var . is_some () { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("change of user name for in use varible {:?}" , global_var) ,)) ; } var_data . user_var = Some (user_var) ; variables . used_user_vars . insert (user_var) ; let sampling_mode = if var_data . sampling_mode == SamplingMode :: Witness { CheckedSamplingMode :: Witness } else { CheckedSamplingMode :: Sample } ; process_step (ctx_in . borrow () , & CheckedProofStep :: UserVar { var : global_var , user_var : Some (CheckedUserVar { user_var , sampling_mode , new_var : true , }) , } ,) ? ; Ok (()) }
};
}
