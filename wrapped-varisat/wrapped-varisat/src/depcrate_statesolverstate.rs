// Generated macro for SolverState (struct)
macro_rules! Depcrate_stateSolverState {
() => {
// Module: crate::state
// Provides: {"SolverState"}
// Dependencies: {}
# [doc = " Miscellaneous solver state."] # [doc = ""] # [doc = " Anything larger or any larger group of related state variables should be moved into a separate"] # [doc = " part of [`Context`](crate::context::Context)."] pub struct SolverState { pub sat_state : SatState , pub formula_is_empty : bool , # [doc = " Whether solve was called at least once."] pub solver_invoked : bool , pub state_is_invalid : bool , pub solver_error : Option < SolverError > , }
};
}
