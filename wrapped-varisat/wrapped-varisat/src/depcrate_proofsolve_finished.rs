// Generated macro for solve_finished (function)
macro_rules! Depcrate_proofsolve_finished {
() => {
// Module: crate::proof
// Provides: {"solve_finished"}
// Dependencies: {}
# [doc = " Called before solve returns to flush buffers and to trigger delayed unit conflict processing."] # [doc = ""] # [doc = " We flush buffers before solve returns to ensure that we can pass IO errors to the user."] pub fn solve_finished < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP)) { flush_proof (ctx . borrow ()) ; if let Some (checker) = & mut ctx . part_mut (ProofP) . checker { let result = checker . self_check_delayed_steps () ; handle_self_check_result (ctx . borrow () , result) ; } }
};
}
