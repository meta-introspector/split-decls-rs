// Generated macro for handle_io_errors (function)
macro_rules! Depcrate_proofhandle_io_errors {
() => {
// Module: crate::proof
// Provides: {"handle_io_errors"}
// Dependencies: {}
# [doc = " Handle io errors during proof writing."] fn handle_io_errors < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP) , result : io :: Result < () > ,) { if let Err (io_err) = result { ctx . part_mut (SolverStateP) . solver_error = Some (SolverError :: ProofIoError { cause : io_err }) ; * ctx . part_mut (ProofP) = Proof :: default () ; } }
};
}
