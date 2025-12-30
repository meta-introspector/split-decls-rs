// Generated macro for SolverError (enum)
macro_rules! Depcrate_solverSolverError {
() => {
// Module: crate::solver
// Provides: {"SolverError"}
// Dependencies: {}
# [doc = " Possible errors while solving a formula."] # [derive (Debug , Error)] # [non_exhaustive] pub enum SolverError { # [error ("The solver was interrupted")] Interrupted , # [error ("Error in proof processor: {}" , cause)] ProofProcessorError { # [source] cause : Error , } , # [error ("Error writing proof file: {}" , cause)] ProofIoError { # [source] cause : io :: Error , } , }
};
}
