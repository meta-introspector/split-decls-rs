// Generated macro for check_change_sampling_mode (function)
macro_rules! Depcrate_statecheck_change_sampling_mode {
() => {
// Module: crate::state
// Provides: {"check_change_sampling_mode"}
// Dependencies: {}
# [doc = " Check a ChangeSamplingMode step"] fn check_change_sampling_mode < 'a > (mut ctx : partial ! (Context <'a >, mut ClausesP , mut ProcessingP <'a >, mut VariablesP , CheckerStateP ,) , var : Var , sample : bool ,) -> Result < () , CheckerError > { ensure_var (ctx . borrow () , var) ; let mut ctx_in = ctx ; let (variables , ctx) = ctx_in . split_part_mut (VariablesP) ; let var_data = & mut variables . var_data [var . index ()] ; let sampling_mode = if sample { SamplingMode :: Sample } else { SamplingMode :: Witness } ; if var_data . sampling_mode != sampling_mode { var_data . sampling_mode = sampling_mode ; if let Some (user_var) = var_data . user_var { let sampling_mode = if var_data . sampling_mode == SamplingMode :: Witness { CheckedSamplingMode :: Witness } else { CheckedSamplingMode :: Sample } ; process_step (ctx_in . borrow () , & CheckedProofStep :: UserVar { var , user_var : Some (CheckedUserVar { user_var , sampling_mode , new_var : false , }) , } ,) ? ; } else if sampling_mode == SamplingMode :: Sample { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("cannot sample hidden variable {:?}" , var) ,)) ; } } Ok (()) }
};
}
