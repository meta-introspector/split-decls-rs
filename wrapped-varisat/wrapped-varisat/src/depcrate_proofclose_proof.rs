// Generated macro for close_proof (function)
macro_rules! Depcrate_proofclose_proof {
() => {
// Module: crate::proof
// Provides: {"close_proof"}
// Dependencies: {}
# [doc = " Stop writing proof steps."] pub fn close_proof < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP , VariablesP) ,) { add_step (ctx . borrow () , true , & ProofStep :: End) ; flush_proof (ctx . borrow ()) ; ctx . part_mut (ProofP) . format = None ; ctx . part_mut (ProofP) . target = BufWriter :: new (Box :: new (sink ())) ; }
};
}
