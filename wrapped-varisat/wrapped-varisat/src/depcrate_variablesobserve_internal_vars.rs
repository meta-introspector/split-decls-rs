// Generated macro for observe_internal_vars (function)
macro_rules! Depcrate_variablesobserve_internal_vars {
() => {
// Module: crate::variables
// Provides: {"observe_internal_vars"}
// Dependencies: {}
# [doc = " Turns all hidden vars into witness vars and returns them."] pub fn observe_internal_vars < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , mut VariablesP) ,) -> Vec < Var > { let mut result = vec ! [] ; let mut variables = ctx . part_mut (VariablesP) ; for global_index in 0 .. variables . global_watermark () { let global = Var :: from_index (global_index) ; let var_data = & variables . var_data [global . index ()] ; if ! var_data . deleted && var_data . sampling_mode == SamplingMode :: Hide { let user = set_sampling_mode (ctx . borrow () , global , SamplingMode :: Witness) . unwrap () ; result . push (user) ; variables = ctx . part_mut (VariablesP) ; } } result }
};
}
