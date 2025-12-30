// Generated macro for handle_self_check_result (function)
macro_rules! Depcrate_proofhandle_self_check_result {
() => {
// Module: crate::proof
// Provides: {"handle_self_check_result"}
// Dependencies: {}
# [doc = " Handle results of on the fly checking."] # [doc = ""] # [doc = " Panics when the proof is incorrect and aborts solving when a proof processor produced an error."] fn handle_self_check_result < 'a > (mut ctx : partial ! (Context <'a >, mut ProofP <'a >, mut SolverStateP) , result : Result < () , CheckerError > ,) { match result { Err (CheckerError :: ProofProcessorError { cause }) => { ctx . part_mut (SolverStateP) . solver_error = Some (SolverError :: ProofProcessorError { cause }) ; * ctx . part_mut (ProofP) = Proof :: default () ; } Err (err) => { log :: error ! ("{}" , err) ; if let CheckerError :: CheckFailed { debug_step , .. } = err { if ! debug_step . is_empty () { log :: error ! ("failed step was {}" , debug_step) } } panic ! ("self check failure") ; } Ok (()) => () , } }
};
}
