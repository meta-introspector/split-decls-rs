// Generated macro for ensure_sampling_var (function)
macro_rules! Depcrate_variablesensure_sampling_var {
() => {
// Module: crate::variables
// Provides: {"ensure_sampling_var"}
// Dependencies: {}
# [doc = " Check that var is a sampling user var and create new variables as necessary."] pub fn ensure_sampling_var (mut ctx : partial ! (Context , mut ClausesP , mut VariablesP , CheckerStateP) , var : Var ,) -> Result < () , CheckerError > { ensure_var (ctx . borrow () , var) ; let variables = ctx . part_mut (VariablesP) ; if variables . var_data [var . index ()] . sampling_mode != SamplingMode :: Sample { return Err (CheckerError :: check_failed (ctx . part (CheckerStateP) . step , format ! ("variable {:?} is not a sampling variable" , var) ,)) ; } Ok (()) }
};
}
