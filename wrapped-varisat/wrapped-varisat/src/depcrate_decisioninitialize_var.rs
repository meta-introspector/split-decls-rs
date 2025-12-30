// Generated macro for initialize_var (function)
macro_rules! Depcrate_decisioninitialize_var {
() => {
// Module: crate::decision
// Provides: {"initialize_var"}
// Dependencies: {}
# [doc = " Initialize decision heuristics for a new variable."] pub fn initialize_var (mut ctx : partial ! (Context , mut VsidsP) , var : Var , available : bool) { ctx . part_mut (VsidsP) . reset (var) ; if available { make_available (ctx . borrow () , var) ; } }
};
}
