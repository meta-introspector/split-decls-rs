// Generated macro for CheckerState (struct)
macro_rules! Depcrate_stateCheckerState {
() => {
// Module: crate::state
// Provides: {"CheckerState"}
// Dependencies: {}
# [doc = " A checker for unsatisfiability proofs in the native varisat format."] # [derive (Default)] pub struct CheckerState { # [doc = " Current step number."] pub step : u64 , # [doc = " Whether unsatisfiability was proven."] pub unsat : bool , # [doc = " Whether an end of proof step was checked."] ended : bool , # [doc = " Last added irredundant clause id."] # [doc = ""] # [doc = " Sorted and free of duplicates."] previous_irred_clause_id : Option < u64 > , # [doc = " Last added irredundant clause literals."] previous_irred_clause_lits : Vec < Lit > , # [doc = " Current assumptions, used to check FailedAssumptions and Model"] assumptions : Vec < Lit > , }
};
}
