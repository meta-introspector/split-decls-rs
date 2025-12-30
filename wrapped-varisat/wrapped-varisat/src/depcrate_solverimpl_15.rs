// Generated macro for impl_15 (impl)
macro_rules! Depcrate_solverimpl_15 {
() => {
// Module: crate::solver
// Provides: {"impl_15"}
// Dependencies: {}
impl SolverError { # [doc = " Whether a Solver instance can be used after producing such an error."] pub fn is_recoverable (& self) -> bool { matches ! (self , SolverError :: Interrupted) } }
};
}
