// Generated macro for global_from_user (function)
macro_rules! Depcrate_variablesglobal_from_user {
() => {
// Module: crate::variables
// Provides: {"global_from_user"}
// Dependencies: {}
# [doc = " Maps a user variable into a global variable."] # [doc = ""] # [doc = " If no matching global variable exists a new global variable is allocated."] pub fn global_from_user < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , mut VariablesP) , user : Var , require_sampling : bool ,) -> Var { let variables = ctx . part_mut (VariablesP) ; if user . index () > variables . user_watermark () { for index in variables . user_watermark () .. user . index () { global_from_user (ctx . borrow () , Var :: from_index (index) , false) ; } } let variables = ctx . part_mut (VariablesP) ; match variables . global_from_user () . get (user) { Some (global) => { if require_sampling && variables . var_data [global . index ()] . sampling_mode != SamplingMode :: Sample { panic ! ("witness variables cannot be constrained") ; } global } None => { let global = match variables . var_data . get (user . index ()) { Some (var_data) if var_data . deleted => user , None => user , _ => variables . next_unmapped_global () , } ; * variables . var_data_global_mut (global) = VarData :: user_default () ; variables . global_from_user_mut () . insert (global , user) ; variables . global_freelist . remove (& global) ; variables . user_freelist . remove (& user) ; proof :: add_step (ctx . borrow () , false , & ProofStep :: UserVarName { global , user : Some (user) , } ,) ; global } } }
};
}
