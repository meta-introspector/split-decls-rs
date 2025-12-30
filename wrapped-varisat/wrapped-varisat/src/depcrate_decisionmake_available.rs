// Generated macro for make_available (function)
macro_rules! Depcrate_decisionmake_available {
() => {
// Module: crate::decision
// Provides: {"make_available"}
// Dependencies: {}
# [doc = " Make a variable available for decisions."] pub fn make_available (mut ctx : partial ! (Context , mut VsidsP) , var : Var) { ctx . part_mut (VsidsP) . make_available (var) ; }
};
}
