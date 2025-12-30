// Generated macro for flush_proof (function)
macro_rules! Depcrate_proofflush_proof {
() => {
// Module: crate::proof
// Provides: {"flush_proof"}
// Dependencies: {}
# [doc = " Flush buffers used for writing proof steps."] pub fn flush_proof < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP)) { let result = ctx . part_mut (ProofP) . target . flush () ; handle_io_errors (ctx . borrow () , result) ; }
};
}
